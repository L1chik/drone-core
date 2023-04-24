#![no_std]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(impl_trait_projections)]

use embassy_stm32::i2c::{Error, I2c, Instance};
use embedded_hal::i2c::ErrorType;

use legacy_hal::blocking as legacy;

pub trait DroneI2c {
    async fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error>;
    async fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), Error>;
    async fn write_read(&mut self, addr: u8, data: &[u8], buffer: &mut [u8]) -> Result<(), Error>;
}

pub struct AsI2c<'d, T: Instance> {
    i2c: I2c<'d, T>,
}

impl<'d, T: Instance> AsI2c<'d, T> {
    pub fn new(i2c: I2c<'d, T>) -> Self {
        Self { i2c }
    }
}

impl<'d, T: Instance> DroneI2c for AsI2c<'d, T> {
    async fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
        self.i2c.blocking_read(addr, buffer)
    }

    async fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), Error> {
        self.i2c.blocking_write(addr, data)
    }

    async fn write_read(&mut self, addr: u8, data: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        self.i2c.blocking_write_read(addr, data, buffer)
    }
}

impl<'d, T: Instance, E> ErrorType for AsI2c<'d, T>
    where E: embedded_hal::i2c::Error + 'static,
          T: Instance + legacy::i2c::WriteRead<Error=E>
          + legacy::i2c::Read<Error=E> + embassy_stm32::i2c::TxDma<T>
          + legacy::i2c::Write<Error=E> + embassy_stm32::i2c::RxDma<T>

{ type Error = E; }

impl<'d, T: Instance> legacy::i2c::Read for AsI2c<'d, T> {
    type Error = Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.blocking_read(addr, buffer)
    }
}

impl<'d, T: Instance> legacy::i2c::Write for AsI2c<'d, T> {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.i2c.blocking_write(addr, bytes)
    }
}

impl<'d, T: Instance> legacy::i2c::WriteRead for AsI2c<'d, T> {
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.blocking_write_read(addr, bytes, buffer)
    }
}