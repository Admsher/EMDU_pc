

use rtt_target::rprintln;
use stm32f4xx_hal::gpio::{Pin};



pub fn systemspage<const P: char, const N: u8>(but: &Pin<P, N>,data: &[u8; 32]){
    let messages: [&str; 7 ] = [
    "EGT",
    "Cylinder 1",
    "Cylinder 2",
    "Cylinder 3",
    "Cylinder 4",
    "Warning Lane",
    "Lane B",
];

 rprintln!("You are on the Systems Page!!!");
 if !but.is_low() {
    rprintln!(" ID | Data               | Description                          ");
    rprintln!("----|--------------------|-------------------------------------");
    for i in 0..7 {
        rprintln!(" {:?} | {:?} | ", messages[i], data[i]);
        rprintln!("----|--------------------|-------------------------------------");
    }
}
    
}
