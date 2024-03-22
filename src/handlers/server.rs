use std::net::Ipv4Addr;
use std::io::{BufReader, Cursor, Read, Write};
use std::thread::{sleep};
use std::time::Duration;
use enet::*;
use anyhow::Context;

use base64::{engine, alphabet, Engine as _};

use std::{fs};
use std::path::Path;
use base64::prelude::BASE64_STANDARD;

use colored::Colorize;

use plist::{Data, Date, Dictionary, Value};
use plist::Integer;
use plist;
use crate::handlers::chunk::Chunk;
use crate::handlers::dynamic_objects::{DynamicObject, DynamicObjectHandler};
use crate::handlers::ServerInformation::ServerInformation;
use crate::handlers::WorldHeartbeat::WorldHeartbeat;
use crate::handlers::Compression::GZIP;
use crate::handlers::dynamic_objects::free_block::{FreeBlock, ItemType};
use crate::handlers::dynamic_objects::workbench::{InteractionObjectType, Workbench, WorkbenchType};

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

    let blocks_gzipped = GZIP::encode(&dummy_blocks).unwrap();
    let light_blocks_gzipped = GZIP::encode(&dummy_light_blocks).unwrap();

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

                println!("Received '{:02x}({}) on channel {}", packet_info.packet_type, packet_info.packet_type, packet_info.channel_id);
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
                    },
                    0x03 => { // request world fragment
                        let x = i8::from_be_bytes([packet_info.raw_data[0]]);
                        let y = i8::from_be_bytes([packet_info.raw_data[1]]) - 20; // this fixes stuff i think
                        println!("[server]: fragment [x={x}], y={y}]");

                        let fragment_data = encode_world_fragment(x, y);

                        send_data(&fragment_data, sender.clone(), 0).unwrap();
                        //println!("sent fragment: {}", {bytes_to_hex_string(&fragment_data)});

                        let spawn_portal = Workbench {
                            unique_id: 6111,
                            x: 11695,
                            y: 743,

                            interaction_object_type: InteractionObjectType::Workbench as u8,
                            object_type: WorkbenchType::BasicPortal as u8,
                            object_variant: 0
                        };
                        let mut compressed_data = spawn_portal.encode_and_compress();
                        compressed_data.insert(0, 7);
                        send_data(&compressed_data, sender.clone(), 0).unwrap();

                        // 2
                        let new_workbench = Workbench {
                            unique_id: 6112,
                            x: 11697,
                            y: 742,

                            interaction_object_type: InteractionObjectType::Workbench as u8,
                            object_type: WorkbenchType::Workbench as u8,
                            object_variant: 1
                        };
                        let mut compressed_data = new_workbench.encode_and_compress();
                        compressed_data.insert(0, 7);
                        send_data(&compressed_data, sender.clone(), 0).unwrap();

                        let new_freeblock = FreeBlock {
                            x: 11695,
                            y: 745,
                            unique_id: 6113,

                            item_type: ItemType::WorkbenchWorkbench as u16,
                            data_a: 10000,
                            data_b: 0,
                            fall_speed: 0,
                            x_velocity: 0,
                            y_velocity: 0,
                            has_subitems: false,
                            hovers: true
                        };
                        let mut compressed_data = new_freeblock.encode_and_compress();
                        compressed_data.insert(0, 0x07);
                        send_data(&compressed_data, sender.clone(), 0).unwrap();
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
                        let dummy_entry = "H4sIAAAAAAAAE0sqyMksLjEwuMjIFJdSmZeYm5nsn5SVmlxSvICDW4oBDBgZITQDMwMqkAYAPiHLuD4AAAA=";
                        let mut dummy_buf = Vec::new();
                        BASE64_STANDARD.decode_vec(dummy_entry, &mut dummy_buf).unwrap();

                        let mut blockhead_files_dict = plist::Dictionary::new();
                        blockhead_files_dict.insert("d972d19b9bb03dc2429ce6337165aa85_blockheads".to_owned(), Value::Data(dummy_buf));

                        let mut dummy_found_items = Vec::new();
                        BASE64_STANDARD.decode_vec("YnBsaXN0MDDUAQIDBAUGBwhYJG9iamVjdHNZJGFyY2hpdmVyWCR2ZXJzaW9uVCR0b3ClCQoLDA1fEA9OU0tleWVkQXJjaGl2ZXISAAGGoNEOD1UkbnVsbNMQERITFBXSERYXGNIZGhsc0hkaHR5aZm91bmRJdGVtc4ABXE5TUmFuZ2VDb3VudFYkY2xhc3NbTlNSYW5nZURhdGEQAoAEgAJXTlMuZGF0YYADRgCuAoAISFgkY2xhc3Nlc1okY2xhc3NuYW1loxwfIF1OU011dGFibGVEYXRhox4hIF8QEU5TTXV0YWJsZUluZGV4U2V0Vk5TRGF0YVhOU09iamVjdFpOU0luZGV4U2V0AAgAEQAaACQALQAyADgASgBPAFIAWABfAGQAaQBuAHkAewCIAI8AmwCdAJ8AoQCpAKsAsgC7AMYAygDYANwA8AD3AQAAAAAAAAACAQAAAAAAAAAiAAAAAAAAAAAAAAAAAAABCw==".to_owned(), &mut dummy_found_items).unwrap();

                        let mut dummy_found_items_v2 = Vec::new();
                        BASE64_STANDARD.decode_vec("H4sIAAAAAAAAA+uXE2cAAT1GIMHCwOABpACHU2jKFAAAAA==".to_owned(), &mut dummy_found_items_v2).unwrap();

                        let mut blockheads_data_dict = plist::Dictionary::new();
                        blockheads_data_dict.insert(
                            String::from("blockheadFiles"),
                            Value::Dictionary(blockhead_files_dict)
                        );
                        blockheads_data_dict.insert("foundItems".to_owned(), Value::Data(dummy_found_items));
                        blockheads_data_dict.insert("foundItems_v2".to_owned(), Value::Data(dummy_found_items_v2));

                        let mut cursor = Cursor::new(Vec::new());
                        plist::to_writer_binary(&mut cursor, &mut blockheads_data_dict).unwrap();
                        let mut data = cursor.into_inner();
                        data.insert(0, 0x06);

                        plist::to_file_xml("./6_sent.xml", &blockheads_data_dict);

                        send_data(&data, sender.clone(), 0).unwrap();
                    },
                    0x0e => { // create blockheda
                        // none of these extra bytes seem to be necessary..?
                        println!("0x0e data: [{}]", packet_info.hex_string);
                        send_data(&[0x0f, 0x0, 0x0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00].to_vec(), sender.clone(), 0).unwrap();
                    },
                    0x18 => {
                        let heartbeat = WorldHeartbeat {
                            world_time: 3600.0,
                            no_rain_timer: 0.0,
                            fast_forward: false,
                            local_paused: false,
                            all_paused: false,
                            pvp_disabled: false,
                            credit: 0.0
                        };

                        let mut data = heartbeat.encode();
                        data.insert(0, 0x17);

                        send_data(&data, sender.clone(), 0).unwrap();
                    },
                    0x35 => { // resend welcome message
                        let mut server_information_plist = plist::Dictionary::new();

                        match &server_info.welcome_message {
                            Some(welcome_message) => {
                                server_information_plist.insert(
                                    String::from("welcomeMessage"),
                                    Value::String(welcome_message.clone())
                                );
                            }
                            None => ()
                        };

                        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
                        plist::to_writer_binary(&mut cursor, &server_information_plist).unwrap();
                        let mut uncompressed_data = cursor.into_inner();
                        uncompressed_data.insert(0, 0x36);

                        send_data(&uncompressed_data, sender.clone(), 0).unwrap()
                    }
                    0x0b => { // unknown
                        let first_byte = packet_info.raw_data[0];

                        let res = GZIP::decode(&packet_info.raw_data[1..].to_vec()).unwrap();
                        let cursor = Cursor::new(res);
                        let p_list = plist::Value::from_reader(cursor).unwrap();

                        //println!("first byte: {first_byte}");

                        let request_array = p_list.as_array().unwrap();
                        for element in request_array {
                            let data = element.as_data().unwrap();
                            //println!("0x0b inner data: {:02x?}", element);
                        };
                    }
                    0x20 => { // UpdatePlayerActionsAndState
                        let res = GZIP::decode(&packet_info.raw_data).unwrap();

                        let cursor = Cursor::new(res);
                        let p_list = plist::Value::from_reader(cursor).unwrap();
                        let dict = p_list.as_dictionary().unwrap();

                        //println!("0x20 decoded: {:?}", dict);
                        p_list.to_file_xml("./0x20.xml").unwrap()
                    }
                    0x0a => { // request create dynamic objects
                        let dynamic_object_type_id = packet_info.raw_data[0];
                        let string_object_type = DynamicObjectHandler::get_name_from_id(dynamic_object_type_id);

                        println!("client wants to create dynamic objects! [type:{}]", string_object_type);

                        let res = GZIP::decode(&packet_info.raw_data[1..].to_vec()).unwrap();
                        let cursor = Cursor::new(res);
                        let p_list = plist::Value::from_reader(cursor).unwrap();

                        let request_array = p_list.as_array().unwrap();
                        for element in request_array {
                            let data = element.as_data().unwrap();
                            println!("request data: {:02x?}", data);

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