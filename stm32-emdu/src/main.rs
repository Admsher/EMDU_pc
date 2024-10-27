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
mod test;


#[entry]
fn main() -> ! {
    let mut data: [u8;32] = [0;32];
    let mut page = 0;

    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let temperature_pin = gpioa.pa0.into_analog();
    let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());
    let rcc = dp.RCC.constrain();
    rcc.cfgr.use_hse(8.MHz()).freeze();
    let gpioc = dp.GPIOC.split();
    let button1 = gpioc.pc7.into_pull_up_input();
    let backbutton = gpioc.pc4.into_pull_up_input();
    let button2 = gpioc.pc3.into_pull_up_input();

    let gpiob = dp.GPIOB.split();
    let mut can1 = {
        let rx = gpiob.pb8;
        let tx = gpiob.pb9;
        let can = dp.CAN1.can((tx, rx));

        bxcan::Can::builder(can)
            .set_bit_timing(0x00050000)
            .enable()
    };

    let mut filters = can1.modify_filters();
    filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    drop(filters);

    let mut can = can1;
   
    // let gpioa = dp.GPIOA.split();
    // let temperature_pin = gpioa.pa0.into_analog();

    // let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());

    // let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
  
   
    // let tx_pin = gpioa.pa2.into_alternate();

    // let mut tx = dp
    //     .USART2
    //     .tx(
    //         tx_pin,
    //         Config::default()
    //             .baudrate(115200.bps())
    //             .wordlength_8()
    //             .parity_none(),
    //         &clocks,
    //     )
    //     .unwrap();

    static R0: f64 = 100000.0;
    static B: f64 = 4275.0; 

    rtt_init_print!();

    loop {
        let sample = adc.convert(&temperature_pin, SampleTime::Cycles_480);

        delay(50000);
        let  volt: f64 = sample as f64/4094 as f64 *3.00;

 
   


        let receive=block!(can.receive()).unwrap();

        let equal = match receive.id() {
                Standard(s) => s.as_raw() as u16 == 0x500,
                Extended(e) => e.as_raw() as u32 == 0x500  as u32, // note that extended Id is 32-bit
            };
          
        if equal{
            for i in 0..receive.data().unwrap().len(){
                data[i] = *receive.data().unwrap().get(i).unwrap();
                

            }
        }
     
        
      if page == 0 {
        if button2.is_low() {
            page = 2; 
            rprintln!("Info page");
                  
                    wait_for_release(&button2);
                    infopage::infopage(&button2, &data);
         
        }
        else if button1.is_low() {
            page = 1; 
            rprintln!("Systems page");
        
                    wait_for_release(&button1);
                    systemspage::systemspage(&button1, &data);
         
        }
    }
     else if page == 1 {
          if backbutton.is_low() {
            wait_for_release(&backbutton);
              page = 0; 
              rprintln!("Returning to home page...");
      
              homepage::homepage(&backbutton, &data);
          }
      } else if page == 2 {
          if backbutton.is_low() {
            wait_for_release(&backbutton);
            
              page = 0; 
              rprintln!("Returning to home page...");
            
              homepage::homepage(&backbutton, &data);
          }
      }
       
    }      
}

fn wait_for_release<const P: char, const N: u8>(but: &Pin<P, N>) {
    while but.is_low() {
        // Do nothing, just wait
    }
}

