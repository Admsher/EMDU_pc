use rtt_target::rprintln;
use stm32f4xx_hal::gpio::{Pin};
use cortex_m::asm::delay;

pub fn homepage<const P: char, const N: u8>(but: &Pin<P, N>, data: &[u8; 32]) -> bool {
    let homepage: [&str; 9] = [
        "Oil Temperature",
        "Oil Pressure",
        "EGT Temperature",
        "Throttle",
        "Fuel Pressure",
        "ECU Voltage",
        "RPM",
        "Coolant Level",
        "Manifold Pressure",
    ];

    // Display the homepage data once
    rprintln!("You are on the homepage!!!");
    
    if but.is_high(){
    rprintln!(" ID | Data               | Description                          ");
    rprintln!("----|--------------------|-------------------------------------");
    
    for i in 0..8 {
        rprintln!(" {:?} | {:?} | ", homepage[i], data[i]);
        rprintln!("----|--------------------|-------------------------------------");
    }
    delay(50);
    }
    if but.is_low(){
        return true;
    }
    true

}


