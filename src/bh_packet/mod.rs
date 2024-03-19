use core::fmt;

use enet::{Packet, Peer};

pub struct BlockheadsPacket<'a> {
    packet_type: u8,
    raw_data: Vec<u8>,
    hex_string: String,
    sender: &'a Peer<'a, ()>
}

pub trait Conversion {
    fn decode(&self);
    fn encode(&self);
}

fn bytes_to_hex_string(data: &Vec<u8>) -> String {
    let mut hex_string = String::new();

    for b in data { // start at 1 cuz byte 0 is the type
        hex_string += &format!("{:02x} ", b);
    }

    return hex_string
}

impl std::fmt::Display for BlockheadsPacket<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(packet type: {:02x})", self.packet_type)
    }
}

impl<'a> BlockheadsPacket<'a> {
    pub fn new(sender: &'a Peer<'a, ()>, packet: &Packet) -> Self {
        let data = packet.data();

        return Self {
            packet_type: data[0],
            raw_data: data[1..].to_vec(),
            hex_string: bytes_to_hex_string(&data[1..].to_vec()),
            sender: sender
        }
    }
}