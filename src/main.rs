#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(int_roundings)]

use defmt::*;
use embassy_executor::{Spawner, task};
use embassy_stm32::{
    pwm::simple_pwm::PwmPin,
    exti::ExtiInput,
    gpio::{Input, Pull, AnyPin, Level, Output, Speed},
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use esc::{
    motor::Fly,
    analog::{AnalogSC, Analog},
};

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

    let blue = Output::new(p.PB7, Level::Low, Speed::Low).degrade();

    spawner.spawn(blink(blue)).unwrap();

    let m1 = PwmPin::new_ch1(p.PE9);
    let m2 = PwmPin::new_ch2(p.PE11);
    let m3 = PwmPin::new_ch3(p.PE13);
    let m4 = PwmPin::new_ch4(p.PE14);
    let mut control = Analog::new(AnalogSC::OneShot42, p.TIM1, (m1, m2, m3, m4));
    control.all_throttle(0.);

    let l0 = 50.;

    let mut level = l0;
    let mut up = true;

    let mut button = ExtiInput::new(Input::new(p.PC13, Pull::Down), p.EXTI13);
    loop {
        button.wait_for_high().await;
        while up && level <= 100. || !up && level >= l0 {
            control.all_throttle(level);
            if up { level += 0.1 } else { level -= 0.1 }
            Timer::after(Duration::from_micros(1)).await;
        }
        up = !up;
    }
}