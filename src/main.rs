// [ChatGPT-4]
// Import the std::time::Duration and std::thread modules for controlling sleep duration
use std::time::Duration;
use std::thread;

// Import the HidApi crate and HidDevice struct
use hidapi::{HidApi, HidDevice};
use ctrlc;

// Define constants for Sony's Vendor ID and DualSense's Product ID
const VENDOR_ID: u16 = 0x054C;
const PRODUCT_ID: u16 = 0x0CE6;

// Define the main function that returns a Result with a trait object for Error
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new HidApi instance
    let api = HidApi::new()?;
    // Open the DualSense controller using the vendor and product IDs
    let device = api.open(VENDOR_ID, PRODUCT_ID)?;

    ctrlc::set_handler(move || {
        // device.close(); // Close the HID device
        println!("Exiting...");
        std::process::exit(0);
    })?;

    // Enter an infinite loop to read the controller input
    loop {
        // Create a mutable buffer to store the input data
        let mut data = [0u8; 64];
        // Read the controller input with a 10ms timeout and store the result
        match device.read_timeout(&mut data, 10) {
            // If data is read successfully, process the input
            Ok(_) => {
                println!("Data: {:?}", &data[..bytes_read]);

                // Process the input and retrieve the resistance values for L2 and R2 triggers
                let (trigger_l, trigger_r) = process_input(&data);
                
                println!("L2: {}, R2: {}", trigger_l, trigger_r);

                // Set the adaptive triggers with the resistance values
                set_adaptive_triggers(&device, trigger_l, trigger_r)?;
            }
            // If there's an error or timeout, continue to the next iteration
            Err(_) => continue,
        }
    }
}

// Define a function to process the input data and return the resistance values for L2 and R2 triggers
fn process_input(data: &[u8]) -> (u8, u8) {
    // Check if the L2 trigger is pressed and set the resistance value accordingly
    let trigger_l = if data[7] > 0 { 127 } else { 0 };
    // Check if the R2 trigger is pressed and set the resistance value accordingly
    let trigger_r = if data[8] > 0 { 127 } else { 0 };
    // Return the resistance values for L2 and R2 triggers
    (trigger_l, trigger_r)
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