#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(int_roundings)]

use defmt::*;
use embassy_executor::{Spawner, task};
use embassy_stm32::{peripherals::{I2C2, DMA2_CH1, USART6},
                    gpio::{AnyPin, Level, Output, Speed},
                    pwm::simple_pwm::PwmPin, i2c::I2c,
                    dma::NoDma, usart::{Config, Parity, Uart}, time::khz, interrupt};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};
use common::AsI2c;
use esc::{
    analog::{Analog, AnalogSC},
    motor::Fly,
};
use icm::{Icm20948, Init, MagEnabled};
use rc_xf::norm_perc;

static THR: Mutex<ThreadModeRawMutex, f32> = Mutex::new(0.);

#[task]
async fn rc_handler(mut rc: Uart<'static, USART6, NoDma, DMA2_CH1>) {
    let mut buf = [0u8; 26];
    loop {
        match rc.read_until_idle(&mut buf).await {
            Ok(_) => if buf[0] == 15 {
                {
                    let mut th = THR.lock().await;
                    *th = norm_perc(&buf, 2);
                    Timer::after(Duration::from_micros(1)).await;
                }
            },
            Err(_) => {}
        };
        Timer::after(Duration::from_micros(1)).await;
    }
}

#[task]
async fn icm_measurements(mut icm: Icm20948<AsI2c<'static, I2C2>, MagEnabled, Init>) {
    loop {
        let all = icm.read_all().await.unwrap();
        println!("ACC: {} {} {}\nGYRO: {} {} {}\nMAG: {} {} {}",
                 all.acc.x, all.acc.y, all.acc.z,
                 all.gyr.x, all.gyr.y, all.gyr.z,
                 all.mag.x, all.mag.y, all.mag.z);
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[task]
async fn blink(mut led: Output<'static, AnyPin>) {
    loop {
        led.set_high();
        info!("*blink*");
        Timer::after(Duration::from_millis(800)).await;
        led.set_low();
        Timer::after(Duration::from_millis(800)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hey there");

    let mut k = Output::new(p.PF10, Level::Low, Speed::Low);
    k.set_high();
    Timer::after(Duration::from_millis(1000)).await;

    let irq_icm = interrupt::take!(I2C2_EV);
    let i2c = I2c::new(p.I2C2, p.PF1, p.PF0, irq_icm, NoDma, NoDma, khz(200), Default::default());
    let asi = AsI2c::new(i2c);

    let icm = Icm20948::new(asi).initialize_9dof().await.unwrap();

    spawner.spawn(icm_measurements(icm)).unwrap();

    let irq_rx = interrupt::take!(USART6);
    let mut c_rx = Config::default();
    c_rx.baudrate = 100_000;
    c_rx.parity = Parity::ParityEven;

    let rc = Uart::new(p.USART6, p.PG9, p.PG14, irq_rx, NoDma, p.DMA2_CH1, c_rx);
    let blue = Output::new(p.PB7, Level::Low, Speed::Low).degrade();

    spawner.spawn(rc_handler(rc)).unwrap();
    spawner.spawn(blink(blue)).unwrap();

    let m1 = PwmPin::new_ch1(p.PE9);
    let m2 = PwmPin::new_ch2(p.PE11);
    let m3 = PwmPin::new_ch3(p.PE13);
    let m4 = PwmPin::new_ch4(p.PE14);
    let mut control = Analog::new(AnalogSC::OneShot42, p.TIM1, (m1, m2, m3, m4));
    control.all_throttle(0.);

    loop {
        Timer::after(Duration::from_micros(1)).await;
        let throttle = THR.lock().await;
        control.all_throttle(*throttle);
    }
}