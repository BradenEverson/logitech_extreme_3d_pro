use evdev::{Device, EventType, InputEvent};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    const VENDOR_ID: u16 = 0x046d;
    const PRODUCT_ID: u16 = 0xc215;

    let mut device = find_joystick(VENDOR_ID, PRODUCT_ID).ok_or("Device not found")?;

    println!(
        "Opened device: {}",
        device.name().unwrap_or("Unnamed device")
    );
    println!("Reading events (Ctrl+C to exit)...\n");

    loop {
        for event in device.fetch_events()? {
            handle_event(event);
        }
    }
}

fn find_joystick(vendor_id: u16, product_id: u16) -> Option<Device> {
    for (_, device) in evdev::enumerate() {
        let id = device.input_id();
        if id.vendor() == vendor_id && id.product() == product_id {
            return Some(device);
        }
    }
    None
}

fn handle_event(event: InputEvent) {
    match event.event_type() {
        EventType::ABSOLUTE => {
            println!("Axis {:?}: {}", event.value(), event.value());
        }
        EventType::KEY => {
            let state = if event.value() == 1 {
                "PRESSED"
            } else {
                "RELEASED"
            };
            println!("Button {:?}: {}", event.code(), state);
        }
        _ => {}
    }
}
