use std::io::{self, Write};
use std::time::Duration;
use serialport::{self, SerialPort};

pub fn button() {
    // Configure the serial port
    let serial_port_name = "/dev/ttyUSB0"; // Change to your serial port (e.g., COM3 on Windows, /dev/ttyUSB0 on Linux)
    let mut port = open_serial_port(serial_port_name).expect("Failed to open port");

    // Start with homepage
    let mut current_page = 0;

    // Placeholder for the 15-bit data
    let mut random_data: Option<u16> = None;

    loop {
        print_page(current_page, random_data);
        let input = get_user_input();

        if current_page == 0 {
            // Home page can go to other pages
            current_page = match input {
                0 => 0,  // Stay on homepage
                1 | 2 | 3 => {
                    // Fetch random data if not already fetched
                    if random_data.is_none() {
                        random_data = Some(receive_random_data(&mut port));
                    }
                    input
                }
                _ => {
                    println!("Invalid input. Please enter 0, 1, 2, or 3.");
                    0
                }
            };
        } else {
            // Other pages can only go back to homepage
            current_page = match input {
                0 => 0,  // Go back to homepage
                _ => {
                    println!("You can only return to the homepage (0).");
                    current_page
                }
            };
        }
    }
}

fn open_serial_port(port_name: &str) -> Result<Box<dyn SerialPort>, serialport::Error> {
    let port = serialport::new(port_name, 9600)
        .timeout(Duration::from_millis(1000))
        .open();

    port
}

fn receive_random_data(port: &mut Box<dyn SerialPort>) -> u16 {
    let mut buffer = [0u8; 2];  // Buffer to hold two bytes of data
    port.read_exact(&mut buffer).expect("Failed to read from port");

    // Reconstruct the 15-bit number from the two bytes
    let upper_byte = buffer[0] & 0x7F;  // Upper 7 bits
    let lower_byte = buffer[1];         // Lower 8 bits
    let random_data = ((upper_byte as u16) << 8) | (lower_byte as u16);

    println!("Received 15-bit random data: {:015b}", random_data);  // Debug output in binary
    random_data
}

fn print_page(page: u32, random_data: Option<u16>) {
    match page {
        0 => println!("You are on the Homepage (0)"),
        1 => {
            if let Some(data) = random_data {
                let bits_page_1 = (data >> 10) & 0x1F;  // Extract the first 5 bits
                println!("You are on the Info Page (1) - First 5 bits: {:05b}", bits_page_1);
            } else {
                println!("You are on the Info Page (1)");
            }
        }
        2 => {
            if let Some(data) = random_data {
                let bits_page_2 = (data >> 5) & 0x1F;  // Extract the second 5 bits
                println!("You are on the Config Page (2) - Middle 5 bits: {:05b}", bits_page_2);
            } else {
                println!("You are on the Config Page (2)");
            }
        }
        3 => {
            if let Some(data) = random_data {
                let bits_page_3 = data & 0x1F;  // Extract the last 5 bits
                println!("You are on the Systems Page (3) - Last 5 bits: {:05b}", bits_page_3);
            } else {
                println!("You are on the Systems Page (3)");
            }
        }
        _ => println!("Invalid Page"),
    }
}

fn get_user_input() -> u32 {
    print!("Enter the page number (0 for Homepage, 1 for Info, 2 for Config, 3 for Systems): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().parse::<u32>().unwrap_or(0)  // Default to 0 if invalid input
}
