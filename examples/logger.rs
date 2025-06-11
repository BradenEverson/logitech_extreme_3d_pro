//! Sample main program that parses all incoming events

use evdev::Device;
use logitech_extreme_3d_pro::joystick::AxisEvent;
use std::error::Error;

/// The Logitech Extreme 3D Pro Vendor ID
pub const VENDOR_ID: u16 = 0x046d;
/// The Logitech Extreme 3D Pro Product ID
pub const PRODUCT_ID: u16 = 0xc215;

fn main() -> Result<(), Box<dyn Error>> {
    let mut device = find_joystick(VENDOR_ID, PRODUCT_ID).ok_or("Device not found")?;

    println!(
        "Opened device: {}",
        device.name().unwrap_or("Unnamed device")
    );

    loop {
        for event in device.fetch_events()? {
            if let Ok(joystick_event) = AxisEvent::try_from(event) {
                println!("{joystick_event:?}");
            }
        }
    }
}

/// Find the joystick by vendor id and product id
fn find_joystick(vendor_id: u16, product_id: u16) -> Option<Device> {
    for (_, device) in evdev::enumerate() {
        let id = device.input_id();
        if id.vendor() == vendor_id && id.product() == product_id {
            return Some(device);
        }
    }
    None
}
