extern crate hidapi;
extern crate itertools;
extern crate serialport;

use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use itertools::Itertools;
use std::{thread, time};
use serialport::SerialPortType;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct GuitarUSBEvent {
    whammy_bar: u16,
    slider_thing: u8,
    b2: u8,
    button_mask: u8,
    which_keys: u8,
    dpad: u8,
}

// TODO: make serial events based on input changes

fn check_differences(prev: &GuitarUSBEvent, curr: &GuitarUSBEvent) -> u8 {
    let mut res = 0;
    if(prev.whammy_bar != curr.whammy_bar) {
        println!("Whammy: {}", curr.whammy_bar);
        res = res | 1 << 0;
    }
    if(prev.slider_thing != curr.slider_thing) {
        println!("Slider: {}", curr.slider_thing);
        res = res | 1 << 1;
    }
    if(prev.b2 != curr.b2) {
        println!("B2: {}", curr.b2);
        res = res | 1 << 2;
    }
    if(prev.button_mask != curr.button_mask) {
        println!("Button Mask: 0b{:08b}", curr.button_mask);
        res = res | 1 << 3;
    }
    if(prev.which_keys != curr.which_keys) {
        println!("Which Buttons: 0x{:x}", curr.which_keys);
        res = res | 1 << 4;
    }
    if(prev.dpad != curr.dpad) {
        println!("Dpad: 0x{:x}", curr.dpad);
        res = res | 1 << 5;
    }

    res
}

fn parse_guitar_data(data: &[u8]) -> GuitarUSBEvent {
    GuitarUSBEvent {
        whammy_bar: ((data[3] as u16) <<8) | (data[2] as u16),//((data[2] as u16) << 8) & (data[3] as u16),
        slider_thing: data[4],
        b2: data[5],
        button_mask: data[6],
        which_keys: data[7],
        dpad: data[8],
    }
}

fn poll_loop(device: &hidapi::HidDevice, mut port: std::boxed::Box<dyn serialport::SerialPort>) {
    let mut done = false;
    let mut data = [0 as u8;12];

    let mut cur_event:GuitarUSBEvent;
    let mut prev_event = GuitarUSBEvent{
        whammy_bar: 0,
        slider_thing: 0,
        b2: 0,
        button_mask: 0,
        which_keys: 0,
        dpad: 0,
    };

    
    let mut now = Instant::now();
    let mut elapsed = now.elapsed().as_millis();
    let mut changes = 0;

    while !done {
        device.read(&mut data);
        cur_event = parse_guitar_data(&data);
        changes = check_differences(&prev_event, &cur_event);

        elapsed = now.elapsed().as_millis();
        if ((changes | 0x1) == 1) && elapsed < 50 && (cur_event.whammy_bar < 0xFF00 && cur_event.whammy_bar > 1) { // if only whammy change and too soon 
            //skip
            continue;
        }
        port.write(&data);
        now = Instant::now();
        
        prev_event = cur_event;
    }

}

fn main() {
    println!("Hello, world!");

    let api = hidapi::HidApi::new().unwrap();
    let devices = api.devices();
    let mut name = String::new();

    if let Ok(ports) = serialport::available_ports() {
        match ports.len() {
            0 => println!("No ports found."),
            1 => println!("Found 1 port:"),
            n => println!("Found {} ports:", n),
        };

        for p in ports {
            println!("  {}", p.port_name);
            match p.port_type {
                SerialPortType::UsbPort(info) => {
                    if (info.vid == 0x16c0) {
                        name = p.port_name.clone();
                        break;
                    }
                }
                SerialPortType::BluetoothPort => {
                    println!("    Type: Bluetooth");
                }
                SerialPortType::PciPort => {
                    println!("    Type: PCI");
                }
                SerialPortType::Unknown => {
                    println!("    Type: Unknown");
                }
            }
        }

    };

    let port = serialport::open(&name).unwrap();

    match api.open(0x1BAD, 0x0002) {
        Ok(device) => {
            println!("Got it!");
            poll_loop(&device, port);
        },
        Err(e) => {
            println!("{:?}", e);    
            println!("Error opening device");
        }
    }

}
