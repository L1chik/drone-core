use embassy_stm32::peripherals::TIM1;
use embassy_stm32::pwm::simple_pwm::{Ch1, Ch2, Ch3, Ch4, PwmPin};


type Motor<T> = PwmPin<'static, TIM1, T>;
pub type Motors = (Motor<Ch1>, Motor<Ch2>, Motor<Ch3>, Motor<Ch4>);

#[derive(Copy, Clone)]
pub enum Quad {
    M1,
    M2,
    M3,
    M4,
}

pub trait Fly {
    type ESC: Copy;

    fn map4_2(&self, val: f32, lim: f32) -> u16;
    fn set_duty(&mut self, motor: Self::ESC, duty: u16) -> u16;
    fn throttle(&mut self, motor: Self::ESC, thr: f32);
    fn all_throttle(&mut self, thr: f32);
}