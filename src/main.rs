// [ChatGPT-4]
// // Import the std::time::Duration and std::thread modules for controlling sleep duration
// use std::time::Duration;
// use std::thread;

// Import the HidApi crate and HidDevice struct
use hidapi::{HidApi, HidDevice};
use ctrlc;

// Define constants for Sony's Vendor ID and DualSense's Product ID
const VENDOR_ID: u16 = 0x054C;
const PRODUCT_ID: u16 = 0x0CE6;

use std::io::{stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = HidApi::new()?;
    let device = api.open(VENDOR_ID, PRODUCT_ID)?;

    ctrlc::set_handler(move || {
        println!("Exiting...");
        std::process::exit(0);
    })?;

    let num_rows = 11;
    let num_columns = 6;
    let row_offset: usize = 1; // Number of rows down to start printing

    let newline_str = "\n".repeat(num_rows+1);
    print!("{}", newline_str);

    let display_data = |data: &[u8]| {
        for row in 0..num_rows {
            for column in 0..num_columns {
                let index = row * num_columns + column;
                if index < data.len() {
                    print!("{:2}: {:<4} ", index, data[index]);
                }
            }
            print!("\n");
        }
    };

    // Print initial empty lines
    for _ in 0..row_offset {
        println!();
    }

    loop {
        let mut data = [0u8; 64];
        match device.read_timeout(&mut data, 10) {
            Ok(bytes_read) if bytes_read > 0 => {
                print!("\x1B[{}F", num_rows + row_offset); // Move the cursor up by 'num_rows + row_offset' lines
                display_data(&data);
                print!("\x1B[{}E", num_rows); // Move the cursor down by 'num_rows' lines
                stdout().flush()?; // Flush the output to ensure it's displayed
            }
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                continue;
            }
        }
    }
}

// Define a function to set the adaptive triggers with the given resistance values
fn set_adaptive_triggers(device: &HidDevice, trigger_l: u8, trigger_r: u8) -> Result<(), Box<dyn std::error::Error>> {
    // Create a data array with the resistance values for L2 and R2 triggers
    let data = [0x05, 0xFF, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, trigger_l, trigger_r];
    // Write the data array to the DualSense controller
    device.write(&data)?;
    // Return an Ok result
    Ok(())
}