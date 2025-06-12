//! Raspberry Pi Stepper Motor Mover Demo

use evdev::Device;
use logitech_extreme_3d_pro::joystick::{Axis, AxisEvent};
use rppal::gpio::Gpio;
use std::error::Error;
use std::thread;
use std::time::Duration;

const VENDOR_ID: u16 = 0x046d;
const PRODUCT_ID: u16 = 0xc215;

const STEP_PIN: u8 = 4;
const DIR_PIN: u8 = 5;

const STEP_DELAY_US: u64 = 1000;
const DEADZONE_LOW: i32 = 450;
const DEADZONE_HIGH: i32 = 574;
const MAX_STEPS: u32 = 50;

fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let mut step_pin = gpio.get(STEP_PIN)?.into_output();
    let mut dir_pin = gpio.get(DIR_PIN)?.into_output();

    let mut device = find_joystick(VENDOR_ID, PRODUCT_ID).ok_or("Joystick not found")?;
    println!(
        "Opened device: {}",
        device.name().unwrap_or("Unnamed device")
    );

    loop {
        for event in device.fetch_events()? {
            if let Ok(axis_event) = AxisEvent::try_from(event) {
                if let AxisEvent::Axis(Axis::StickLeftRight, value) = axis_event {
                    let (direction, steps) = if value < DEADZONE_LOW {
                        (false, ((DEADZONE_LOW - value) as u32 / 10).min(MAX_STEPS))
                    } else if value > DEADZONE_HIGH {
                        (true, ((value - DEADZONE_HIGH) as u32 / 10).min(MAX_STEPS))
                    } else {
                        (false, 0)
                    };

                    if steps > 0 {
                        if direction {
                            dir_pin.set_high();
                        } else {
                            dir_pin.set_low();
                        }

                        for _ in 0..steps {
                            step_pin.set_high();
                            thread::sleep(Duration::from_micros(STEP_DELAY_US));
                            step_pin.set_low();
                            thread::sleep(Duration::from_micros(STEP_DELAY_US));
                        }
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}

fn find_joystick(vendor_id: u16, product_id: u16) -> Option<Device> {
    evdev::enumerate()
        .find(|(_, device)| {
            let id = device.input_id();
            id.vendor() == vendor_id && id.product() == product_id
        })
        .map(|(_, device)| device)
}
