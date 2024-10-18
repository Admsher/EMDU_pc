use bxcan::Frame;
use rtt_target::rprintln;
use stm32f4xx_hal::gpio::{Pin};

pub fn infopage<const P: char, const N: u8>(but: &Pin<P, N>, data: [u8; 32]) -> bool {
    let info_page: [&str; 17] = [
    "RPM",
    "Throttle",
    "Manifold Pressure",
    "Coolant Temperature",
    "EGT Cylinder 1",
    "EGT Cylinder 2",
    "EGT Cylinder 3",
    "EGT Cylinder 4",
    "Oil Pressure",
    "Fuel Pressure",
    "Fuel Flow",
    "Manifold Temperature",
    "ECU Voltage",
    "Ambient Pressure",
    "Ambient Temperature",
    "Warning Lane",
    "Lane B",
];

    
    while !but.is_low() {
        
        rprintln!("You are on the info page!!!");
        rprintln!(" ID | Data               | Description                          ");
        

        rprintln!("----|--------------------|-------------------------------------");
        for i in 0..17 {
            rprintln!(" {:?} | {:?} | ", info_page[i], data[i]);
            rprintln!("----|--------------------|-------------------------------------");
        }
       
    }
    true
}
