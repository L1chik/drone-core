use embassy_stm32::peripherals::TIM1;
use embassy_stm32::pwm::Channel::{Ch1, Ch2, Ch3, Ch4};
use embassy_stm32::pwm::simple_pwm::SimplePwm;
use embassy_stm32::time::{Hertz, hz, khz};
use embassy_time::Duration;

use crate::motor::{Fly, Motors, Quad};
use crate::motor::Quad::{M1, M2, M3, M4};

pub enum AnalogSC {
    Standard,
    OneShot125,
    OneShot42,
    Multishot,
}

pub struct Analog {
    pwm: SimplePwm<'static, TIM1>,
    base: f32,
}

impl From<(TIM1, Duration, Hertz, Motors)> for Analog {
    fn from(p: (TIM1, Duration, Hertz, Motors)) -> Self {
        let base = (Self::N * p.1.as_micros() as u32) as f32 /
            (Self::N * 1000).div_ceil(p.2.0 << 8) as f32;
        let mut pwm = SimplePwm::new(p.0, None, None, None, None, p.2);
        pwm.enable(Ch1);
        pwm.enable(Ch2);
        pwm.enable(Ch3);
        pwm.enable(Ch4);
        Analog { pwm, base }
    }
}

impl Analog {
    const N: u32 = 16;

    pub fn new(protocol: AnalogSC, timer: TIM1, motors: Motors) -> Analog {
        match protocol {
            AnalogSC::Standard => (timer, Duration::from_micros(1000), hz(400), motors).into(),
            AnalogSC::OneShot125 => (timer, Duration::from_micros(125), khz(2), motors).into(),
            AnalogSC::OneShot42 => (timer, Duration::from_micros(42), khz(8), motors).into(),
            AnalogSC::Multishot => (timer, Duration::from_micros(12), khz(32), motors).into()
        }
    }
}

impl Fly for Analog {
    type ESC = Quad;

    fn map4_2(&self, val: f32, lim: f32) -> u16 {
        (self.base + self.base * val / lim) as u16
    }

    fn set_duty(&mut self, motor: Self::ESC, duty: u16) -> u16 {
        match motor {
            Self::ESC::M1 => self.pwm.set_duty(Ch1, duty),
            Self::ESC::M2 => self.pwm.set_duty(Ch2, duty),
            Self::ESC::M3 => self.pwm.set_duty(Ch3, duty),
            Self::ESC::M4 => self.pwm.set_duty(Ch4, duty)
        }
        duty
    }

    fn throttle(&mut self, motor: Self::ESC, thr: f32) {
        self.set_duty(motor, self.map4_2(thr.clamp(0., 100.), 100.));
    }

    fn all_throttle(&mut self, thr: f32) {
        [M1, M2, M3, M4].iter().for_each(|&t| self.throttle(t, thr))
    }
}