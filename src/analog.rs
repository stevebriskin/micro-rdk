#![allow(dead_code)]
use core::cell::RefCell;
use esp_idf_hal::adc::{AdcChannelDriver, AdcDriver, Attenuation};
use esp_idf_hal::gpio::ADCPin;
use std::rc::Rc;

pub struct FakeAnalogReader {
    name: String,
    value: u16,
}

impl FakeAnalogReader {
    pub fn new(name: String, value: u16) -> Self {
        Self { name, value }
    }
    fn internal_name(&self) -> String {
        self.name.clone()
    }
    fn internal_read(&self) -> anyhow::Result<u16> {
        Ok(self.value)
    }
}

impl AnalogReader<u16> for FakeAnalogReader {
    type Error = anyhow::Error;
    fn name(&self) -> String {
        self.internal_name()
    }
    fn read(&mut self) -> Result<u16, Self::Error> {
        self.internal_read()
    }
}

pub struct Esp32AnalogReader<'a, T: ADCPin, ATTEN> {
    channel: AdcChannelDriver<'a, T, ATTEN>,
    driver: Rc<RefCell<AdcDriver<'a, T::Adc>>>,
    name: String,
}

pub trait AnalogReader<Word> {
    type Error;
    fn read(&mut self) -> Result<Word, Self::Error>;
    fn name(&self) -> String;
}

impl<'a, T: ADCPin, ATTEN> Esp32AnalogReader<'a, T, ATTEN>
where
    ATTEN: Attenuation<T::Adc>,
{
    pub fn new(
        name: String,
        channel: AdcChannelDriver<'a, T, ATTEN>,
        driver: Rc<RefCell<AdcDriver<'a, T::Adc>>>,
    ) -> Self {
        Self {
            name,
            channel,
            driver,
        }
    }
    fn inner_read(&mut self) -> anyhow::Result<u16> {
        self.driver
            .borrow_mut()
            .read(&mut self.channel)
            .map_err(|e| anyhow::anyhow!(format!("error while reading analog reader {}", e)))
    }
    fn inner_name(&self) -> String {
        self.name.clone()
    }
}

impl<'a, T: ADCPin, ATTEN> AnalogReader<u16> for Esp32AnalogReader<'a, T, ATTEN>
where
    ATTEN: Attenuation<T::Adc>,
{
    type Error = anyhow::Error;
    fn read(&mut self) -> Result<u16, Self::Error> {
        self.inner_read()
    }
    fn name(&self) -> String {
        self.inner_name()
    }
}
