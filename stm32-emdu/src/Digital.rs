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
};
use bxcan::filter::Mask32;
use bxcan::{Fifo, Frame, StandardId};
use rtt_target::{rprintln, rtt_init_print};
// #[path=r"../src/CAN.rs"] mod can;
// #[path=r"../src/Analog.rs"] mod analog;

pub fn button() -> ! {
    let mut page = 0;
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let temperature_pin = gpioa.pa0.into_analog();
    let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());

    

    let gpioc = dp.GPIOC.split();
    let button1 = gpioc.pc7.into_pull_up_input(); // Assuming pull-up
    let backbutton = gpioc.pc4.into_pull_up_input(); // Assuming pull-up
    let button2 = gpioc.pc3.into_pull_up_input(); // Assuming pull-up

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
            .set_bit_timing(0x001c_0000)
            .enable()
    };
    let mut filters = can1.modify_filters();
    filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
     // Drop filters to leave filter configuration mode.
     drop(filters);

     // Select the interface.
     let mut can = can1;





    loop {
        let sample = adc.convert(&temperature_pin, SampleTime::Cycles_480);
        let receive=(can.receive()).unwrap();
        let voltage =sample as f64/4094.00*3.00;
   
        if page == 0 {
            if button2.is_low(){
                // Button 2 pressed
                page = 2; // Transition to info page
                rprintln!("Transitioning to info page...");
                // Wait until button is released
                wait_for_release(&button2);
                infopage(&button2,receive);
            }
            if button1.is_low() {
                // Button 1 pressed
                page = 1; // Transition to system page
                rprintln!("Systems Page");
                wait_for_release(&button1);
                systemspage(&button1,voltage);

            }
        } else if page == 1 {
            if backbutton.is_low() {
                // Back button pressed
                page = 0; // Transition back to home page
                rprintln!("Returning to home page...");
                // Wait until button is released
                wait_for_release(&backbutton);
            }
        } else if page == 2 {
            if backbutton.is_low() {
                // Back button pressed
                page = 0; // Transition back to home page
                rprintln!("Returning to home page...");
                // Wait until button is released
                wait_for_release(&backbutton);
            }
        }

        // Toggle LED for visual feedback, optional
        // led.toggle();
    }
}

fn wait_for_release<const P: char, const N: u8>(but: &Pin<P, N>) {

    while but.is_low() {
        // Do nothing, just wait
    }
}

fn infopage<const P: char, const N: u8>(but: &Pin<P, N>,data:Frame) -> bool {

    // while !but.is_low() {
    //     // Do nothing, just wait
    // }

   
 
    //
  

    
    while !but.is_low() {
       rprintln!("You are on the info page!!!");
      
       rprintln!("{:?}",data.data().unwrap().get(2).unwrap())
    }

    
    true
}

fn systemspage<const P: char, const N: u8>(but: &Pin<P, N>,volt:f64) -> bool {
   
    // while !but.is_low() {
    //     // Do nothing, just wait
    // }

    
    
    // Wait for the button to be released
    while !but.is_low() {
      
       
        rprintln!("{}",volt);
    }

    // Indicate that the action has occurred
    true
}

fn homepage<const P: char, const N: u8>(but: &Pin<P, N>) -> bool {
 
    while !but.is_low() {
        // Do nothing, just wait
    }

    rprintln!("You are on the home page!!!");

    // Wait for the button to be released
    while but.is_low() {
        // Do nothing, just wait
    }

    // Indicate that the action has occurred
    true
}
