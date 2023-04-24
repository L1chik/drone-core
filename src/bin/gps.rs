#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::str::from_utf8;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    init,
    usart::Config,
    interrupt,
};
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::Uart;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = init(Default::default());
    info!("Hey there");

    let mut config = Config::default();
    config.baudrate = 9600;

    let irq = interrupt::take!(USART2);
    let mut dm = [0u8; 1280];
    let mut bx = Uart::new(
        p.USART2,
        p.PD6, p.PD5, irq, NoDma, p.DMA1_CH5, config
    );
    loop {
        let ch = bx.read_until_idle(&mut dm).await.unwrap_or(0);
        if ch == 0 { error!("Error") }
        println!("{}", from_utf8(&dm).unwrap_or("X"));
    }
}