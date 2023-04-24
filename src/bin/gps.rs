#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::str::from_utf8;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    init,
    usart::{BufferedUart, Config, State},
    interrupt,
};
use embedded_io::asynch::BufRead;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = init(Default::default());
    info!("Hey there");

    let mut config = Config::default();
    config.baudrate = 9600;


    let mut state = State::new();
    let irq = interrupt::take!(USART6);
    let mut dm = [0u8; 1280];
    let mut bx = BufferedUart::new(
        &mut state, p.USART6,
        p.PG9, p.PG14,
        irq, &mut [], &mut dm, config,
    );
    loop {
        let buf = bx.fill_buf().await.unwrap();
        let data = from_utf8(buf).unwrap_or("x");
        info!("{}", data);
        let n = buf.len();
        bx.consume(n);
    }
}