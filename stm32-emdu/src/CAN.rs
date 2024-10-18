#![no_main]
#![no_std]

use core::ops::Index;

use panic_halt as _;

use bxcan::filter::Mask32;
use bxcan::{Fifo, Frame, StandardId,ExtendedId};
use cortex_m_rt::entry;
use nb::block;
use rtt_target::{rprintln,rtt_init_print};
use stm32f4xx_hal::{pac, prelude::*};
use bxcan::Id::{Extended,Standard};

pub fn CAN()-> [u8;32] {
    let dp = pac::Peripherals::take().unwrap();
    let mut data: [u8;32] = [0;32];

    let rcc = dp.RCC.constrain();

    // To meet CAN clock accuracy requirements an external crystal or ceramic
    // resonator must be used. The blue pill has a 8MHz external crystal.
    // Other boards might have a crystal with another frequency or none at all.
    rcc.cfgr.use_hse(8.MHz()).freeze();

    let gpiob = dp.GPIOB.split();
    let mut can1 = {
        let rx = gpiob.pb8;
        let tx = gpiob.pb9;

        // let can = Can::new(dp.CAN1, (tx, rx));
        // or
        let can = dp.CAN1.can((tx, rx));

        bxcan::Can::builder(can)
            // APB1 (PCLK1): 8MHz, Bit rate: 500kBit/s, Sample Point 87.5%
            // Value was calculated with http://www.bittiming.can-wiki.info/
            .set_bit_timing( 0x00050000)
            .enable()
    };
    rtt_init_print!();
    // Configure filters so that can frames can be received.
    let mut filters = can1.modify_filters();
    filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());

    let _can2 = {
        let tx = gpiob.pb13;
        let rx = gpiob.pb12;

        let can = dp.CAN2.can((tx, rx));

        let can2 = bxcan::Can::builder(can)
            // APB1 (PCLK1): 8MHz, Bit rate: 500kBit/s, Sample Point 87.5%
            // Value was calculated with http://www.bittiming.can-wiki.info/
            .set_bit_timing(  0x00050000
            )
            .enable();

        // A total of 28 filters are shared between the two CAN instances.
        // Split them equally between CAN1 and CAN2.
        filters.set_split(14);
        let mut slave_filters = filters.slave_filters();
        slave_filters.enable_bank(14, Fifo::Fifo0, Mask32::accept_all());
        can2
    };

    // Drop filters to leave filter configuration mode.
    drop(filters);

    // Select the interface.
    let mut can = can1;
    //let mut can = can2;

    // Echo back received packages in sequence.
    // See the `can-rtfm` example for an echo implementation that adheres to
    // correct frame ordering based on the transfer id.
    let mut test: [u8; 8] = [0; 8];
    let mut count: u8 = 0;
    let id: u16 = 0x500;

    test[1] = 1;
    test[2] = 2;
    test[3] = 3;
    test[4] = 4;
    test[5] = 5;
    test[6] = 6;
    test[7] = 7;
    loop {
        // rprintln!("Runaway");
        let receive=block!(can.receive()).unwrap();
        let equal = match receive.id() {
                Standard(s) => s.as_raw() as u16 == 0x121212,
                Extended(e) => e.as_raw() as u32 == 0x121212  as u32, // note that extended Id is 32-bit
            };
        if equal{
            for i in 0..receive.data().unwrap().len(){
                data[i] = *receive.data().unwrap().index(i);
                
            }
        }
    }
    data
}