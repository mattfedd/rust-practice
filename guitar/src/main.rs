extern crate hidapi;
extern crate itertools;

use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use itertools::Itertools;
use std::{thread, time};

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

fn check_differences(prev: &GuitarUSBEvent, curr: &GuitarUSBEvent) {
    if(prev.whammy_bar != curr.whammy_bar) {println!("Whammy: {}", curr.whammy_bar);}
    if(prev.slider_thing != curr.slider_thing) {println!("Slider: {}", curr.slider_thing);}
    if(prev.b2 != curr.b2) {println!("B2: {}", curr.b2);}
    if(prev.button_mask != curr.button_mask) {println!("Button Mask: 0b{:08b}", curr.button_mask);}
    if(prev.which_keys != curr.which_keys) {println!("Which Buttons: 0x{:x}", curr.which_keys);}
    if(prev.dpad != curr.dpad) {println!("Dpad: 0x{:x}", curr.dpad);}
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

fn poll_loop(device: &hidapi::HidDevice) {
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

    while !done {
        device.read(&mut data);
        cur_event = parse_guitar_data(&data);
        check_differences(&prev_event, &cur_event);
        // println!("Data is {:x?}", usb_event);
        prev_event = cur_event;
    }

}

fn main() {
    println!("Hello, world!");

    let api = hidapi::HidApi::new().unwrap();
    let devices = api.devices();

    match api.open(0x1BAD, 0x0002) {
        Ok(device) => {
            println!("Got it!");
            poll_loop(&device);
        },
        Err(e) => {
            println!("{:?}", e);    
            println!("Error opening device");
        }
    }

}
