
use std::io::{BufReader, Read, Write};
use std::fs;
use std::net::Ipv4Addr;
use std::io::Cursor;
use std::thread::sleep;
use std::time::Duration;
use colored::Colorize;


use plist::{Integer, Value};

extern crate plist;
extern crate enet;
extern crate libflate;
use enet::*;
use anyhow::Context;

use crate::handlers::KickReason::KickReason;

use libflate::gzip::*;

struct PacketInfo {
    packet_type: u8,
    raw_data: Vec<u8>,
    hex_string: String,
    channel_id: u8
}

fn bytes_to_hex_string(data: &Vec<u8>) -> String {
    let mut hex_string = String::new();

    for b in data { // start at 1 cuz byte 0 is the type
        hex_string += &format!("{:02x} ", b);
    }

    return hex_string
}

pub struct ClientInformation {
    pub alias: String,
    pub game_center_id: String,
    pub minor_version: u64,
    pub local: bool,
    pub udid_new: String,
    pub mic_or_speaker_on: bool,
    pub icloud_id: String,
    pub player_id: String,
    pub voice_connected: bool
}

fn send_data(data: &Vec<u8>, mut peer: Peer<'_, ()>, channel_id: u8) -> Result<(), Error> {
    let packet = Packet::new(data, PacketMode::ReliableSequenced).unwrap();
    println!("sent packet {:02x?}", data[0]);
    return peer.send_packet(packet, channel_id)
}

fn print_chunk(chunk: Vec<u8>) {
    let mut blocks_read = 0;
    let mut i = 0;

    let mut print_entries = Vec::new();
    let mut cur_row = Vec::new();

    let mut chunk_index = 0;
    let mut block_index = 0;

    while i < chunk.len() {
        if i % 64 == 0 {
            let byte = chunk[i];
            let displayed = match byte {
                0x1 => "S".truecolor(69, 69, 69),  // STONE
                0x2 => "-".black(),                // AIR
                0x3 => "W".blue(),                 // WATER
                0x4 => "I".blue(),                 // ICE
                0x5 => "S".white(),                // SNOW
                0x6 => "D".truecolor(140, 73, 28), // DIRT
                // SAND
                // WOOD
                // MINED_STONE
                // RED_BRICK
                // LIMESTONE
                // MINED_LIMESTONE
                // MARBLE
                // MINED_MARBLE
                0x10 => "T".cyan(),
                // SAND_STONE
                // MINED_SAND_STONE
                // RED_MARBLE
                // MINED_RED_MARBLE
                // GLASS
                0x19 => "P".blue(), // SPAWN_PORTAL_BASE
                // GOLD_BLOCK
                0x1b => "G".green(), // GRASS_DIRT
                0x1c => "S".white(), // SNOW_DIRT
                _ => {
                    println!("{:02x}", byte);
                    String::from("?").white()
                }
            };

            cur_row.insert(block_index, displayed);

            block_index += 1;

            blocks_read += 1;
            if blocks_read % 32 == 0 {
                print_entries.insert(chunk_index, cur_row);
                chunk_index += 1;
                block_index = 0;

                cur_row = Vec::new();
            }
        }
        i += 1;
    }

    print_entries.reverse();
    for chunk in print_entries {
        for block in chunk {
            print!("{}", block);
        }
        println!("")
    }
}

pub fn start(ip: Ipv4Addr, port: u16, clientinfo: ClientInformation) {
    let enet = Enet::new().context("could not initialize enet").unwrap();

    let mut host = enet
        .create_host::<()>(
            None,
            10,
            ChannelLimit::Maximum,
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited
        )
        .context("could not create host").unwrap();

    host.connect(&Address::new(ip, port), 255, 0)
        .context("connect failed!").unwrap();

    loop {
        match host.service(1000).context("service failed").unwrap() {
            Some(Event::Connect(_)) => println!("[client]: new connection!"),
            Some(Event::Disconnect(..)) => println!("[client: disconnected!"),
            Some(Event::Receive {
                ref mut sender,
                channel_id,
                ref packet
            }) => {    
                let packet_info = PacketInfo {
                    packet_type: packet.data()[0],
                    raw_data: packet.data()[1..].to_vec(),
                    hex_string: bytes_to_hex_string(&packet.data()[1..].to_vec()),
                    channel_id: channel_id
                };
    
                println!("Received '{:02x} on channel {}", packet_info.packet_type, packet_info.channel_id);
                //println!(
                //    "[client]: got packet type '{:02x}' on channel {}, hex content: {}",
                //    packet_info.packet_type,
                //    packet_info.channel_id,
                //    packet_info.hex_string
                //);
    
                match packet_info.packet_type {
                    0x23 => {
                        // from first byte, because for some weird unknown reason the 0th byte has to be '&'
                        let cursor = Cursor::new(&packet_info.raw_data[1..]);
                        let reader = BufReader::new(cursor);
    
                        assert_eq!(packet_info.raw_data[0], 0x26, "Invalid WorldId!");
    
                        let client_information_plist = plist::Value::from_reader(reader).unwrap();
                        let dict = client_information_plist.as_dictionary().unwrap();
    
                        client_information_plist.to_file_xml("./plist.xml").unwrap();
    
                        let world_id = dict.get("worldID").unwrap().as_string().unwrap();
                        //let owner_name = dict.get("ownerName").unwrap().as_string().unwrap(); // owner_name exists only on lan worlds
    
                        println!("worldID: {world_id}");

                        // send client information    
                        let mut client_information_plist = plist::Dictionary::new();
                        client_information_plist.insert(
                            String::from("alias"),
                            Value::String(clientinfo.alias.clone())
                        );
                        client_information_plist.insert(
                            String::from("minorVersion"),
                            Value::Integer(Integer::from(clientinfo.minor_version))
                        );
                        client_information_plist.insert(
                            String::from("local"),
                            Value::Boolean(clientinfo.local)
                        );
                        client_information_plist.insert(
                            String::from("udidNew"),
                            Value::String(clientinfo.udid_new.clone())
                        );
                        client_information_plist.insert(
                            String::from("voiceConnected"),
                            Value::Boolean(clientinfo.voice_connected)
                        );
                        client_information_plist.insert(
                            String::from("micOrSpeakerOn"),
                            Value::Boolean(clientinfo.mic_or_speaker_on)
                        );
                        client_information_plist.insert(
                            String::from("iCloudID"),
                            Value::String(clientinfo.icloud_id.clone())
                        );
                        client_information_plist.insert(
                            String::from("playerID"),
                            Value::String(clientinfo.player_id.clone())
                        );

                        let mut cursor = Cursor::new(Vec::new());

                        cursor.write_all(&[0x1f]).unwrap();

                        plist::to_writer_binary(&mut cursor, &client_information_plist).unwrap();

                        let data = cursor.into_inner();
                        send_data(&data, sender.clone(), 0).unwrap();
                        //println!("data sent: {:x?}", data);
                        println!("[client]: sent ClientInformation!");
                    },
                    0x26 => { // Server kicked client
                        let kick_reason = KickReason::from_code(packet_info.raw_data[0]);
                        println!("Server kicked client! Reason: {}", kick_reason.display_message);
                    },
                    0x01 => { // this is server information (gzipped)
                        println!("[client]: got ServerInformation!");

                        let mut decoder = Decoder::new(&packet_info.raw_data[..]).unwrap();
                        let mut server_info_buffer = Vec::new();
                        decoder.read_to_end(&mut server_info_buffer).unwrap();

                        //let cursor = Cursor::new(server_info_buffer);

                        //let server_information_plist = plist::Value::from_reader(reader).unwrap();

                        // request a fragment
                        let fragment_req = [0x03, 0x6d, 0x2d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00].to_vec();

                        send_data(&fragment_req, sender.clone(), 0).unwrap();
                        println!("[client]: request fragment!");
                        
                        //                        // request a fragment
                        let fragment_req = [0x03, 0x6d, 0x2f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00].to_vec();

                        send_data(&fragment_req, sender.clone(), 0).unwrap();
                        println!("[client]: request fragment!");           

                        // request chat messages
                        send_data(&[0x05].to_vec(), sender.clone(), 0).unwrap();

                        sleep(Duration::from_secs(3));
                        send_data(&[0x0e].to_vec(), sender.clone(), 0).unwrap();

                        let fragment_req = [0x03, 0x6d, 0x2e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00].to_vec();

                        send_data(&fragment_req, sender.clone(), 0).unwrap();
                        println!("[client]: request fragment!");       

                        //send_data(&[0x0e].to_vec(), sender.clone(), 0).unwrap();
                    },
                    0x04 => { // receive world fragment
                        let x = u8::from_be_bytes([packet_info.raw_data[0]]);
                        let y = u8::from_be_bytes([packet_info.raw_data[1]]) - 20; // this fixes stuff i think
                        let unknown_short: i16 = i16::from_be_bytes([packet_info.raw_data[2], packet_info.raw_data[3]]); // no idea
                        println!("[client]: received fragment [x={x}], y={y}, short={}]", unknown_short);

                        let cursor = Cursor::new(&packet_info.raw_data[4..]);
                        let reader = BufReader::new(cursor);
        
                        let world_fragment_plist = plist::Value::from_reader(reader).unwrap();
                        let dict = world_fragment_plist.as_dictionary().unwrap();

                        let compressed_blocks = dict.get("t").unwrap().as_data().unwrap();
                        let compressed_light_blocks = dict.get("r").unwrap().as_data().unwrap();

                        let mut decoder = Decoder::new(compressed_blocks).unwrap();
                        let mut blocks = Vec::new();
                        decoder.read_to_end(&mut blocks).unwrap();
                        
                        let mut decoder = Decoder::new(compressed_light_blocks).unwrap();
                        let mut light_blocks = Vec::new();
                        decoder.read_to_end(&mut light_blocks).unwrap();
                        
                        fs::write( format!("./block_{}_{}", x, y), &blocks).unwrap();
                        fs::write(format!("./light_block_{}_{}", x, y), light_blocks).unwrap();

                        //println!("blocks: {:?}", blocks_str);
                        //println!("light blocks: {:?}", light_blocks);

                        print_chunk(blocks);

                        world_fragment_plist.to_file_xml("./0x04.xml").unwrap();
                    },
                    0x1e => { // player list
                        let cursor = Cursor::new(&packet_info.raw_data);
        
                        let client_information_plist = plist::Value::from_reader(cursor).unwrap();
                        //let dict = client_information_plist.as_dictionary().unwrap();
    
                        client_information_plist.to_file_xml("./1e.xml").unwrap();
                    },
                    0x25 => { // message history
                        let cursor = Cursor::new(&packet_info.raw_data);
                        let message_history = plist::Value::from_reader(cursor).unwrap();

                        message_history.to_file_xml("./message_history.xml").unwrap();
                    },
                    0x0f => {
                        print!("0x0f response: [{}]", packet_info.hex_string);
                        send_data(&[0x0a, 0x18, 0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x4b, 0x2a, 0xc8, 0xc9, 0x2c, 0x2e, 0x31, 0x30, 0x58, 0xc8, 0xe8, 0x2f, 0xd0, 0xcc, 0x00, 0x05, 0xeb, 0x75, 0x19, 0x18, 0x9e, 0x33, 0x01, 0x19, 0x75, 0x79, 0x27, 0x18, 0x37, 0x7b, 0xbb, 0x32, 0x32, 0x31, 0x30, 0x5d, 0xec, 0x48, 0xff, 0x3f, 0x41, 0x43, 0xf4, 0xbf, 0x45, 0xce, 0x9a, 0xff, 0x6b, 0xc4, 0x8d, 0xff, 0x33, 0xa0, 0x01, 0x71, 0x86, 0x7a, 0x38, 0x3b, 0x09, 0x6a, 0xe8, 0x45, 0x46, 0xa6, 0x90, 0xbc, 0xc4, 0xdc, 0xd4, 0x70, 0x5f, 0xc7, 0x90, 0x10, 0x0f, 0xd7, 0x70, 0x0e, 0x6e, 0x01, 0x88, 0x02, 0x46, 0x46, 0xa8, 0x4a, 0x66, 0x34, 0x53, 0x24, 0x38, 0xb8, 0xd0, 0x54, 0x30, 0xa1, 0xa9, 0x98, 0x00, 0x00, 0x34, 0x7a, 0x56, 0xc5, 0xb2, 0x00, 0x00, 0x00
                            ].to_vec(), sender.clone(), 0).unwrap();
                    }
                    0x6 => { // blockheads data
                        // First 125 bytes are ?????
                        // Rest are plist

                        let mut cursor = Cursor::new(&packet_info.raw_data[124..]);
                        
                        let data_plist = plist::Value::from_reader(&mut cursor).unwrap();
                        data_plist.to_file_xml("./6.xml").unwrap();

                        let blockhead_files_dict = plist::Dictionary::new();

                        let mut blockheads_data_dict = plist::Dictionary::new();
                        blockheads_data_dict.insert(
                            String::from("blockheadFiles"),
                            Value::Dictionary(blockhead_files_dict)
                        );

                        let mut cursor = Cursor::new(Vec::new());
                        plist::to_writer_xml(&mut cursor, &mut blockheads_data_dict).unwrap();
                        //let data: Vec<u8> = cursor.into_inner();
                        //fs::write("./6_2.xml", data).unwrap();
                    },
                    0x07 => {
                        let unknown_value = packet_info.raw_data[0];
                        let mut decoder = Decoder::new(&packet_info.raw_data[1..]).unwrap();
                        let mut buffer = Vec::new();
                        decoder.read_to_end(&mut buffer).unwrap();

                        let mut cursor = Cursor::new(buffer);
                        let property_list = plist::Value::from_reader(&mut cursor).unwrap();

                        //println!("unknown value: {:02x}", unknown_value);
                        let property_array = property_list.as_array().unwrap();

                        //println!("{}", packet_info.hex_string);

                        for data in property_array {
                            //println!("{:02x?}", data.as_data().unwrap());
                        }
                    }
                    0x09 => {
                        //println!("packet 0x09! {}", packet_info.hex_string);

                        //let mut decoder = Decoder::new(&packet_info.raw_data[..]).unwrap();
                        //let mut buffer = Vec::new();
                        //decoder.read_to_end(&mut buffer).unwrap();

                        //println!("decoded: {:02x?}", buffer);
                    },
                    _ => println!("Unhandled packet! Hex [{}]", packet_info.hex_string)
                }
            },
            _ => ()
        };
    }
}