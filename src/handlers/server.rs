use std::net::Ipv4Addr;
use std::io::{BufReader, Cursor, Read, Write};
use std::thread::{sleep};
use std::time::Duration;
use enet::*;
use anyhow::Context;

use std::{fs};
use std::path::Path;

use colored::Colorize;

use libflate::gzip::*;

use plist::{Date, Dictionary, Value};
use plist::Integer;
use plist;
use crate::handlers::chunk::Chunk;
use crate::handlers::ServerInformation::ServerInformation;
use crate::handlers::WorldHeartbeat::WorldHeartbeat;

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

fn decode_client_info(raw_data: Vec<u8>) -> Dictionary {
    let cursor = Cursor::new(raw_data);
    let reader = BufReader::new(cursor);

    let client_information_plist = plist::Value::from_reader(reader).unwrap();
    let dict = client_information_plist.as_dictionary().unwrap().to_owned();

    client_information_plist.to_file_xml("./proper_clientinfo.plist.xml").unwrap();

    return dict;
}

fn encode_world_fragment(x: i8, y: i8) -> Vec<u8> {
    //let dummy_blocks = fs::read(format!("./block_{}_{}", x, y)).unwrap();
    //let dummy_light_blocks = fs::read(format!("./light_block_{}_{}", x, y)).unwrap();

    let chunk_path = format!("./block_{}_{}", x, y);
    let dummy_blocks = if Path::new(chunk_path.as_str()).exists() { // Found a saved chunk
        fs::read(chunk_path).unwrap()
    } else { // Empty air chunk
        Chunk::new().encode()
    };

    //let dummy_blocks = fs::read("./modified_chunk").unwrap();
    let dummy_light_blocks = fs::read("./light_block_109_27").unwrap();

    //print_chunk(dummy_blocks.clone());

    let mut blocks_encoder: Encoder<Vec<u8>> = Encoder::new(Vec::new()).unwrap();
    blocks_encoder.write_all(&dummy_blocks).unwrap();
    let blocks_gzipped = blocks_encoder.finish().into_result().unwrap();

    let mut light_blocks_encoder: Encoder<Vec<u8>> = Encoder::new(Vec::new()).unwrap();
    light_blocks_encoder.write_all(&dummy_light_blocks).unwrap();
    let light_blocks_gzipped = light_blocks_encoder.finish().into_result().unwrap();

    let curs = Cursor::new(&blocks_gzipped);
    let mut decod = Decoder::new(curs).unwrap();
    let mut res = Vec::new();
    decod.read_to_end(&mut res).unwrap();

    assert_eq!(res, dummy_blocks);

    let mut fragment_dictionary = plist::Dictionary::new();
    fragment_dictionary.insert(
        String::from("t"),
        Value::Data(blocks_gzipped)
    );
    fragment_dictionary.insert(
        String::from("r"),
        Value::Data(light_blocks_gzipped)
    );

    let mut fragment_cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    plist::to_writer_binary(&mut fragment_cursor, &fragment_dictionary).unwrap();
    plist::to_file_xml("./fragment_sent.xml", &fragment_dictionary).unwrap();

    let mut fragment_data = fragment_cursor.into_inner();
    fragment_data.insert(0, 0x04);
    fragment_data.insert(1, x.to_be_bytes()[0]);
    fragment_data.insert(2, (y + 20).to_be_bytes()[0]);
    let short_bytes = i16::to_be_bytes(0);
    fragment_data.insert(3, short_bytes[0]); // no idea
    fragment_data.insert(4, short_bytes[1]);
    println!("{:02x?}", short_bytes);

    return fragment_data;
}

fn encode_world_id() -> Vec<u8> {
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    cursor.write(&[0x23]).unwrap(); // packet id
    cursor.write(&[0x26]).unwrap(); // ampersand, don't ask.

    let mut world_id_dict = plist::Dictionary::new();
    world_id_dict.insert(
        String::from("worldID"),
        Value::String(String::from("9e9ab16b-31c9-66ee-3054-4355b0ab62ed"))
    );

    plist::to_writer_binary(&mut cursor, &world_id_dict).unwrap();

    return cursor.into_inner();
}

fn send_data(data: &Vec<u8>, mut peer: Peer<'_, ()>, channel_id: u8) -> Result<(), Error> {
    let packet = Packet::new(data, PacketMode::ReliableSequenced).unwrap();
    println!("sent packet {:02x?}", data[0]);
    return peer.send_packet(packet, channel_id)
}

fn encode_player_list() -> Vec<u8> {
    let player_list_vec = Vec::new();
    let mut fake_player_dict = plist::Dictionary::new();
    fake_player_dict.insert(
        String::from("alias"),
        Value::String(String::from("fake player"))
    );
    fake_player_dict.insert(
        String::from("playerID"),
        Value::String(String::from("d972d19b9bb03dc2429ce633yiencoa4"))
    );
    fake_player_dict.insert(
        String::from("connected"),
        Value::Boolean(true)
    );
    fake_player_dict.insert(
        String::from("mod"),
        Value::String(String::from("owner"))
    );

    let player_list = plist::Value::Array(player_list_vec);

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    plist::to_writer_binary(&mut cursor, &player_list).unwrap();

    let mut data = cursor.into_inner();
    data.insert(0, 0x1e);

    return data;
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


pub fn start(ip: Ipv4Addr, port: u16, server_info: ServerInformation) {
    let enet = Enet::new().context("could not initialize ENet").unwrap();

    let address = Address::new(ip, port);

    let mut host = enet
        .create_host::<()>(
            Some(&address),
            10,
            ChannelLimit::Maximum,
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited
        )
        .context("could not create host").unwrap();

    loop {
        match host.service(1000).context("service failed").unwrap() {
            Some(Event::Connect(ref mut peer)) => {
                println!("[server]: connection made! {} channels count: {}", peer.address().ip(), peer.channel_count());

                let world_id_encoded = encode_world_id();
                
                send_data(&world_id_encoded, peer.clone(), 0).unwrap();
            },
            Some(Event::Disconnect(..)) => println!("[server]: disconnect!"),
            Some(Event::Receive {
                ref mut sender,
                channel_id,
                ref packet,..
            }) => {
                let packet_info = PacketInfo {
                    packet_type: packet.data()[0],
                    raw_data: packet.data()[1..].to_vec(),
                    hex_string: bytes_to_hex_string(&packet.data()[1..].to_vec()),
                    channel_id: channel_id
                };

                println!("Received '{:02x} on channel {}", packet_info.packet_type, packet_info.channel_id);
                //println!(
                //    "[server]: got packet type '{:02x}' on channel {}, hex content: {}",
                //    packet_info.packet_type,
                //    packet_info.channel_id,
                //    packet_info.hex_string
                //);

                match packet_info.packet_type {
                    0x1f => { // client information
                        println!("[server]: packet is client information!");
                        let client_info = decode_client_info(packet_info.raw_data);
                        println!("[server]: '{}' connected", client_info.get("alias").unwrap().as_string().unwrap());

                        // TODO: mess with the client information

                        // send server information    

                        let mut server_info_data = server_info.encode();
                        server_info_data.insert(0, 0x01);
                        send_data(&server_info_data, sender.clone(), 0).unwrap();

                        let player_list_data: Vec<u8> = encode_player_list();
                        send_data(&player_list_data, sender.clone(), 0).unwrap();

                        sleep(Duration::from_secs(3));
                        let obj_data: Vec<u8> = [0x07, 0xde, 0x80, 0x34, 0x46, 0xa9, 0xf2, 0x02, 0x46, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00].to_vec();
                        send_data(&obj_data, sender.clone(), 0).unwrap();

                        let obj_data: Vec<u8> = [0x07, 0x00, 0x84, 0x34, 0x46, 0xc8, 0xf5, 0x02, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00].to_vec();
                        send_data(&obj_data, sender.clone(), 0).unwrap();

  
                        //send_data(
                        //    &[].to_vec(),
                        //    sender.clone(),
                        //    0
                        //).unwrap();
                    },
                    0x03 => { // request world fragment
                        let x = i8::from_be_bytes([packet_info.raw_data[0]]);
                        let y = i8::from_be_bytes([packet_info.raw_data[1]]) - 20; // this fixes stuff i think
                        println!("[server]: fragment [x={x}], y={y}]");

                        let fragment_data = encode_world_fragment(x, y);

                        send_data(&fragment_data, sender.clone(), 0).unwrap();
                        //println!("sent fragment: {}", {bytes_to_hex_string(&fragment_data)});

                        let obj_data: Vec<u8> = [0x07, 0x0e, 0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x4b, 0x2a, 0xc8, 0xc9, 0x2c, 0x2e, 0x31, 0x30, 0x58, 0xc6, 0xc8, 0xc4, 0xcc, 0xc2, 0xca, 0xe6, 0x2f, 0x60, 0x91, 0xc7, 0xcb, 0x00, 0x06, 0x7b, 0x74, 0x19, 0x18, 0xde, 0x33, 0x01, 0x19, 0xd9, 0x1f, 0xa6, 0x35, 0x46, 0x08, 0x85, 0x32, 0x40, 0x81, 0x34, 0xc3, 0x76, 0x86, 0xeb, 0x0c, 0x5f, 0xfe, 0xc3, 0xf8, 0x0c, 0x8c, 0x0c, 0x6f, 0x19, 0x81, 0x14, 0x50, 0x6b, 0x3e, 0x54, 0xeb, 0x5e, 0x9c, 0x5a, 0x77, 0x33, 0x5c, 0xc6, 0xae, 0xb5, 0x00, 0xaa, 0x75, 0x3b, 0x50, 0xeb, 0x4b, 0x1c, 0x5a, 0x2f, 0x61, 0xd7, 0x5a, 0x08, 0xd5, 0xba, 0x13, 0xa8, 0xf5, 0x03, 0x0e, 0xad, 0x17, 0xb1, 0x6b, 0x2d, 0x82, 0x6a, 0xdd, 0x0d, 0xd4, 0xfa, 0x09, 0xab, 0xd6, 0x7d, 0xb8, 0x1c, 0x7c, 0x8d, 0x0f, 0x11, 0x4c, 0xaf, 0x40, 0x5a, 0x0b, 0x3e, 0x4c, 0x6d, 0x3c, 0x9a, 0xdd, 0x88, 0xd0, 0xba, 0x03, 0x3d, 0x98, 0x58, 0x40, 0x5a, 0x19, 0x38, 0x18, 0xf8, 0x19, 0xbc, 0x18, 0x5a, 0x19, 0x0e, 0x30, 0xfc, 0x66, 0x34, 0x83, 0x48, 0x31, 0x31, 0x42, 0xd5, 0xb0, 0x33, 0xa0, 0x00, 0xc6, 0x42, 0x00, 0xb8, 0xe4, 0x4d, 0xa6, 0x9f, 0x01, 0x00, 0x00].to_vec();
                        send_data(&obj_data, sender.clone(), 0).unwrap();

                        send_data(
                            &[0x07, 0x2d, 0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x4b, 0x2a, 0xc8, 0xc9, 0x2c, 0x2e, 0x31, 0x30, 0x58, 0xc8, 0xe8, 0x2f, 0x90, 0xb8, 0x9b, 0x01, 0x02, 0xd6, 0xeb, 0x32, 0x30, 0x3c, 0x67, 0x62, 0xc0, 0x00, 0x8c, 0x60, 0xb2, 0xe0, 0x38, 0xba, 0x88, 0x7c, 0x37, 0x07, 0x84, 0x2b, 0xec, 0xad, 0x75, 0xe2, 0xa4, 0x8e, 0x9e, 0xa1, 0xc1, 0x8e, 0x03, 0x07, 0x19, 0xd8, 0x7a, 0x7a, 0x78, 0x02, 0xd8, 0x98, 0xb5, 0x1e, 0xb0, 0x30, 0x58, 0xf4, 0x1b, 0xcf, 0xd3, 0x02, 0xca, 0x73, 0x70, 0x41, 0xb5, 0x31, 0x42, 0xf5, 0xa3, 0x5b, 0x92, 0x07, 0x00, 0x31, 0xbc, 0x6a, 0x65, 0x90, 0x00, 0x00, 0x00].to_vec(),
                            sender.clone(),
                            0
                        ).unwrap();
                        send_data(
                            &[0x07, 0x0e, 0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x4b, 0x2a, 0xc8, 0xc9, 0x2c, 0x2e, 0x31, 0x30, 0x58, 0xc2, 0xc8, 0xc4, 0xcc, 0xe2, 0x2f, 0x60, 0xa1, 0x23, 0xce, 0x00, 0x06, 0x7b, 0x74, 0x19, 0x18, 0xde, 0x33, 0x01, 0x19, 0x85, 0xfc, 0x2f, 0x18, 0x19, 0x10, 0x40, 0x9a, 0x61, 0x2f, 0xc3, 0x15, 0x86, 0x2f, 0xff, 0xe1, 0x02, 0x8c, 0x60, 0xc4, 0x00, 0xd4, 0xaa, 0x0b, 0xd5, 0xba, 0x17, 0x8f, 0xd6, 0x0b, 0xd8, 0xb5, 0x5e, 0x86, 0x6a, 0xdd, 0x0d, 0xd4, 0xfa, 0x04, 0x8b, 0x56, 0x16, 0x06, 0x4c, 0x00, 0xd5, 0x7a, 0x05, 0xc9, 0xd6, 0xc7, 0x78, 0xb5, 0x32, 0xa1, 0x68, 0xe5, 0xe0, 0xf5, 0x68, 0xde, 0x07, 0xe5, 0xc2, 0xd4, 0xb3, 0xa2, 0x59, 0xf1, 0x13, 0x00, 0xb3, 0xa0, 0x75, 0x45, 0x1e, 0x01, 0x00, 0x00].to_vec(),
                            sender.clone(),
                            0
                        ).unwrap();
                        send_data(
                            &[0x07, 0x0e, 0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x4b, 0x2a, 0xc8, 0xc9, 0x2c, 0x2e, 0x31, 0x30, 0x58, 0xc8, 0xe8, 0x2f, 0x60, 0xf1, 0x50, 0x9c, 0x01, 0x0c, 0xf6, 0xe8, 0x32, 0x30, 0xbc, 0x65, 0x02, 0x32, 0x66, 0xe9, 0xf4, 0x33, 0x4e, 0xce, 0x4e, 0x62, 0x80, 0x02, 0x16, 0x30, 0xf9, 0xe5, 0x3f, 0x03, 0x43, 0x3d, 0x03, 0x32, 0xe0, 0xe0, 0x82, 0xd0, 0x8c, 0x8c, 0x50, 0x01, 0x26, 0x06, 0x54, 0xe0, 0x0a, 0x00, 0x0c, 0x35, 0xa1, 0x01, 0x67, 0x00, 0x00, 0x00].to_vec(),
                            sender.clone(),
                            0
                        ).unwrap();
                    }
                    0x05 => { // always empty supposedly
                        let mut test_messages = Vec::new();
                        let mut dummy_message = Dictionary::new();
                        dummy_message.insert(
                            String::from("mod"),
                            Value::String(String::from("owner"))
                        );
                        dummy_message.insert(
                            String::from("message"),
                            Value::String(String::from("hellooo"))
                        );
                        dummy_message.insert(
                            String::from("alias"),
                            Value::String(String::from("server"))
                        );
                        dummy_message.insert(
                            String::from("playerID"),
                            Value::String(String::from("d972d19b9bb03dc2429ce6337165aa85"))
                        );
                        dummy_message.insert(
                            String::from("date"),
                            Value::Date(Date::from_xml_format("2024-03-17T22:25:37.210543Z").unwrap())
                        );

                        test_messages.insert(0, Value::Dictionary(dummy_message));

                        let mut msg_plist = plist::Dictionary::new();
                        msg_plist.insert(
                            String::from("messages"),
                            Value::Array(test_messages)
                        );

                        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
                        plist::to_writer_binary(&mut cursor, &msg_plist).unwrap();

                        let mut data = cursor.into_inner();
                        data.insert(0, 0x25);

                        send_data(&data, sender.clone(), 0).unwrap();

                                                // weird thing
                        let blockhead_files_dict = plist::Dictionary::new();

                        let mut blockheads_data_dict = plist::Dictionary::new();
                        blockheads_data_dict.insert(
                            String::from("blockheadFiles"),
                            Value::Dictionary(blockhead_files_dict)
                        );

                        let mut cursor = Cursor::new(Vec::new());
                        plist::to_writer_binary(&mut cursor, &mut blockheads_data_dict).unwrap();
                        let mut data = cursor.into_inner();
                        data.insert(0, 0x06);

                        send_data(&data, sender.clone(), 0).unwrap();
                    },
                    0x0e => { // create blockheda
                        // none of these extra bytes seem to be necessary..?
                        println!("0x0e data: [{}]", packet_info.hex_string);
                        send_data(&[0x0f, 0x0, 0x0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00].to_vec(), sender.clone(), 0).unwrap();
                    },
                    0x18 => {
                        let heartbeat = WorldHeartbeat {
                            world_time: 92.0,
                            no_rain_timer: 0.0,
                            fast_forward: true,
                            local_paused: false,
                            all_paused: false,
                            pvp_disabled: false,
                            credit: 0.0
                        };

                        let mut data = heartbeat.encode();
                        data.insert(0, 0x17);

                        send_data(&data, sender.clone(), 0).unwrap();
                    },
                    0x0a => { // request create objects
                        let unknown_1 = packet_info.raw_data[0];
                        let unknown_2 = packet_info.raw_data[1];
                        //let unknown_3 = packet_info.raw_data[2];

                        println!("client wants to create objects! {:02x?}", packet_info.raw_data);

                        let curs = Cursor::new(&packet_info.raw_data[1..]);
                        let mut decod = Decoder::new(curs).unwrap();
                        let mut res = Vec::new();
                        decod.read_to_end(&mut res).unwrap();

                        let cursor = Cursor::new(res);
                        let p_list = plist::Value::from_reader(cursor).unwrap();

                        let request_array = p_list.as_array().unwrap();
                        for element in request_array {
                            let data = element.as_data().unwrap();
                            println!("request data: {:02x?}", element);

                            let first_stuff = data[..71].to_vec();
                            let rest = data[72..].to_vec();

                            let another_cursor = Cursor::new(rest);
                            let plist_data = plist::Value::from_reader(another_cursor);

                            plist_data.unwrap().to_file_xml("./0x0a_inner").unwrap();
                        }

                        p_list.to_file_xml("./0x0a.xml").unwrap();
                    }
                    _ => println!("Unhandled packet! Hex [{}]", packet_info.hex_string)
                }
            },
            None => (),
        }
    }
}