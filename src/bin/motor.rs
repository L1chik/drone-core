#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(int_roundings)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    init,
    adc::Adc,
    pwm::{Channel::*, simple_pwm::{PwmPin, SimplePwm}},
    time::hz,
};
use embassy_time::Delay;
use {defmt_rtt as _, panic_probe as _};

// Any frequency between [20 and 500) Hz
const FREQ: u32 = 400;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut p = init(Default::default());
    info!("Hey there");
    let mut adc = Adc::new(p.ADC1, &mut Delay);

    let (t, r) = (16000_u32, 64);
    let (motor, ch) = (PwmPin::new_ch3(p.PB0), Ch3);
    let mut pwm = SimplePwm::new(p.TIM3, None, None, Some(motor), None, hz(FREQ));
    pwm.enable(ch);

    let map4_2 = |val, lim, b| (b + b * val as u32 / lim) as u16;
    loop { pwm.set_duty(ch, map4_2(adc.read(&mut p.PA3), 4096, t / t.div_ceil(r * FREQ))); }
}