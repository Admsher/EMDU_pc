use rtt_target::rprintln;
use stm32f4xx_hal::gpio::{Pin};

pub fn homepage<const P: char, const N: u8>(but: &Pin<P, N>,data:[u8;32]) -> bool {
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

    while !but.is_low() {
        rprintln!("You are on the homepage!!!");
        rprintln!(" ID | Data               | Description                          ");
        rprintln!("----|--------------------|-------------------------------------");

        for i in 0..9 {
            rprintln!(" {:?} | {:?} | ", homepage[i], data[i]);
            rprintln!("----|--------------------|-------------------------------------");
        }
    }

    true
}
