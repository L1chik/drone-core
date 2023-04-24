#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{info, println};
use embassy_executor::Spawner;
use embassy_stm32::{
    init, interrupt,
    i2c::I2c,
    dma::NoDma,
    time::khz,
};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use common::{AsI2c, DroneI2c};
use icm::Icm20948;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = init(Default::default());
    info!("Hey there");

    let mut k = Output::new(p.PF10, Level::Low, Speed::Low);
    k.set_high();
    Timer::after(Duration::from_millis(1000)).await;

    let irq = interrupt::take!(I2C2_EV);
    let mut i2c = I2c::new(p.I2C2, p.PF1, p.PF0, irq, NoDma, NoDma, khz(200), Default::default());
    let mut asi = AsI2c::new(&mut i2c);

    let mut data = [0u8; 1];
    asi.write_read(0x68, &[0x00], &mut data).await.unwrap();

    let mut icm = Icm20948::new(asi).initialize_9dof().await.unwrap();

    loop {
        let all = icm.read_all().await.unwrap();
        println!("ACC: {} {} {}\nGYRO: {} {} {}\nMAG: {} {} {}",
                 all.acc.x, all.acc.y, all.acc.z,
                 all.gyr.x, all.gyr.y, all.gyr.z,
                 all.mag.x, all.mag.y, all.mag.z);
        Timer::after(Duration::from_millis(100)).await;
    }
}