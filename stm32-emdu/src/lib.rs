#![no_std]
#![no_main]




use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    can::Can,
    gpio::{Pin, Input, Output, PushPull},
    pac,
    prelude::*,
    adc::{config::AdcConfig, config::SampleTime, Adc},
    serial::config::Config,
};
use bxcan::{filter::Mask32, Data};
use bxcan::{Fifo, Frame, StandardId};
use rtt_target::{rprintln, rtt_init_print};
use nb::block;
use bxcan::Id::{Extended,Standard};
use libm::log;
use nb::Error; 
mod infopage;
mod systemspage;
mod homepage;
use cortex_m::asm::delay;
// mod CAN;
// mod Analog;
// mod Digital;
use defmt::{debug, info, error};


#[entry]
fn main() -> ! {
loop{}

}