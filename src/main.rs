use std::{convert::TryInto, env};
use std::time::Duration;

use serial::prelude::*;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud9600,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone,
};

fn main() {
    for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg).unwrap();

        loop {
            println!("{:?}", read_anemometer(&mut port).unwrap());
        }
    }
}

fn read_anemometer<T: SerialPort>(port: &mut T) -> Option<[f32; 3]> {
    port.configure(&SETTINGS).ok();
    port.set_timeout(Duration::from_secs(1)).ok();

    let mut buf: Vec<u8> = (0..12).collect();

    port.read_exact(&mut buf).ok();

    let byte_slice: &[u8] = &buf[0..4];
    let arr: &[u8; 4] = byte_slice.try_into().unwrap();
    let val1 = f32::from_le_bytes(*arr);
    let byte_slice: &[u8] = &buf[4..8];
    let arr: &[u8; 4] = byte_slice.try_into().unwrap();
    let val2 = f32::from_le_bytes(*arr);
    let byte_slice: &[u8] = &buf[8..12];
    let arr: &[u8; 4] = byte_slice.try_into().unwrap();
    let val3 = f32::from_le_bytes(*arr);
    Some([val1, val2, val3])
}