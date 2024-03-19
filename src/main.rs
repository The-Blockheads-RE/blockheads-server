use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};
use std::io::{Cursor, Read, Write};
use std::net::Ipv4Addr;

pub mod handlers;

use handlers::{block, chunk::*};
use handlers::server;
use handlers::server::ServerInformation;
use handlers::client;
use handlers::client::ClientInformation;

use libflate::gzip::*;

// ClientInformation = 0x1F
// KeepAlive = 0x18
// RequestChatHistory = 0x05
// RequestCreateObjects = 0x0a
// RequestRemoveObjects = 0x0c
// RequestWorldFragment = 0x03
// UpdatePlayerActionsAndState = 0x20
// UpdatePlayerInventory = 0x21

use macroquad::{prelude::*, ui};
use macroquad::ui::{hash, root_ui, widgets};

const PIXEL_SIZE: f32 = 20.0;

async fn gui() {
    let blocks = fs::read("./block_109_27").unwrap();
    let chunk = Chunk::decode(blocks);
    chunk.print();

    let text_x_position = ((CHUNK_WIDTH as f32) * PIXEL_SIZE);
        //root_ui().input_text(90, "input", &mut inp);

    loop {
        //let mut text1: String = String::from("hey");

        //if true {
            //widgets::Window::new(hash!(), vec2(text_x_position, 540.0), vec2(300.0, 40.0))
            //.titlebar(false)
            //.movable(true)
            //.ui(&mut *root_ui(), |ui| {

            //    ui.editbox(3, Vec2::new(1000.0, 1000.0), &mut text1);
                    //.size(Vec2::new(1000.0, 1000.0))
                    //.position(Vec2::new(0.0, 0.0))
                    //.label(None, "hii")
                    //.ui(ui, &mut inp);
                //ui.tree_node(3, "editbox 1", |ui| {
                //    let mut text1 = String::from("hi");
                //    //ui.label(None, "this is editbox");
                //    ui.editbox(hash!(), vec2(285.0, 165.0), &mut text1);
                //});
            //});

            //next_frame().await;
            //continue;
        //}
        clear_background(BLUE);

        //macroquad::window::request_new_screen_size(32.0 * 20.0, (32.0 * 20.0) + 40.0);

        //root_ui().window(hash!(), Vec2::new(20.0, 20.0), Vec2::new(450.0, 200.0), |ui| {
        //    let (mouse_x, mouse_y) = mouse_position();
        //    ui.label(None, &format!("Mouse position: {} {}", mouse_x, mouse_y));
        //    draw_text(&format!("{}, {}", mouse_x, mouse_y), 20.0, 20.0, 30.0, DARKGRAY);
        //
        //    widgets::Group::new(hash!(), Vec2::new(200.0, 90.0))
        //        .position(Vec2::new(240.0, 0.0))
        //        .ui(ui, |ui| {
        //            ui.label(None, "hi");
        //        });
        //});

        let mut y: usize = 0;

        let mut row_buffer = Vec::new();

        while y < CHUNK_HEIGHT {
            let mut x: usize = 0;

            let mut row = Vec::new();

            while x < CHUNK_WIDTH {
                let block = chunk.get_block(x, y).unwrap();

                row.insert(row.len(), block);
                x += 1
            }

            row_buffer.insert(y, row);
            y += 1
        }

        row_buffer.reverse();

        let mut y = 0;
        for row in row_buffer {
            let mut x = 0;

            while x < row.len() {
                let f_x = x as f32;
                let f_y = y as f32;

                let block = row[x];

                let mut background_color = block.get_back_wall_color();
                background_color.a = 0.7;

                let pos = Vec2::new((f_x * PIXEL_SIZE), (f_y * PIXEL_SIZE) + 0.0);

                draw_rectangle(pos.x, pos.y, PIXEL_SIZE, PIXEL_SIZE, background_color);
                draw_rectangle(pos.x, pos.y, PIXEL_SIZE, PIXEL_SIZE, block.get_color());

                if block.sub_type_index.get() != 0 {
                    draw_line(pos.x, pos.y, pos.x + PIXEL_SIZE, pos.y + PIXEL_SIZE, 3.0, PURPLE);
                    draw_line(pos.x + PIXEL_SIZE, pos.y, pos.x, pos.y + PIXEL_SIZE, 3.0, PURPLE);
                }

                //if (block.source_data[08] != 0) { draw_rectangle((f_x * 20.0), (f_y * 20.0) + 40.0, 20.0, 20.0, RED); }
                //if (block.source_data[40] != 0) { draw_rectangle((f_x * 20.0), (f_y * 20.0) + 40.0, 20.0, 20.0, PURPLE); }
                //if (block.source_data[20] != 0) { draw_rectangle((f_x * 20.0), (f_y * 20.0) + 40.0, 20.0, 20.0, DARKGREEN); }
                x += 1
            }

            y += 1;
        }

        let (mouse_x, mouse_y) = mouse_position();
        let hit_mouse_x = (mouse_x / PIXEL_SIZE).floor();
        let hit_mouse_y = (((mouse_y) / PIXEL_SIZE)).floor();

        let block_option = chunk.get_block(
            hit_mouse_x as usize,
            ((CHUNK_HEIGHT as f32) - hit_mouse_y - 1.0) as usize,
        );

        match block_option {
            Some(block) => {
                let mut color = block.get_color();
                let mut background_color = block.get_back_wall_color();        
        
                color.a = 1.0;
                background_color.a = 1.0;

                let font_size = 30.0;

                let artificial_light_color = Color::from_rgba(block.initial_data[14], block.initial_data[16], block.initial_data[18], 255);
        
                draw_text(&format!("mouse {}, {}", hit_mouse_x, hit_mouse_y), text_x_position, 20.0, font_size, BLACK);
                draw_text(&format!("block '{}'", block.get_name()), text_x_position, 40.0, font_size, color);
                draw_text(&format!("background '{}'", block.get_back_wall_name()), text_x_position, 60.0, font_size, background_color);
                draw_text(&format!("zone_type '{}'", block.initial_data[2]), text_x_position, 80.0, font_size, BLACK);
                draw_text(&format!("subtype '{}'", block.get_subtype_name()), text_x_position, 100.0, font_size, BLACK);
                draw_text(&format!("partial_content_left '{}'", block.initial_data[4]), text_x_position, 120.0, font_size, BLACK);
                draw_text(&format!("damage '{}'", block.gather_progress.get()), text_x_position, 140.0, font_size, BLACK);
                draw_text(&format!("light '{}'", block.initial_data[6]), text_x_position, 160.0, font_size, BLACK);
                draw_text(&format!("sun_light '{}'", block.initial_data[7]), text_x_position, 180.0, font_size, BLACK);
                draw_text(&format!("season_offset '{}'", block.initial_data[8]), text_x_position, 200.0, font_size, BLACK);
                draw_text(&format!("explored_fraction '{}'", block.initial_data[9]), text_x_position, 220.0, font_size, BLACK);
                draw_text(&format!("terrain_slow_factor '{}'", block.initial_data[10]), text_x_position, 240.0, font_size, BLACK);
                draw_text(&format!("foreground_contents '{}'", block.initial_data[11]), text_x_position, 260.0, font_size, BLACK);
                draw_text(&format!("background_contents '{}'", block.initial_data[12]), text_x_position, 280.0, font_size, BLACK);
                draw_text(&format!("artificial_light_r '{}'", block.initial_data[14]), text_x_position, 300.0, font_size, artificial_light_color);
                draw_text(&format!("artificial_light_g '{}'", block.initial_data[16]), text_x_position, 320.0, font_size, artificial_light_color);
                draw_text(&format!("artificial_light_b '{}'", block.initial_data[18]), text_x_position, 340.0, font_size, artificial_light_color);

                //draw_text(&format!("foreground '{}'", block.source_data[8]), text_x_position, 100.0, font_size, BLACK);
                //draw_text(&format!("subtype '{}'", block.get_subtype_name()), text_x_position, 120.0, font_size, BLACK);
                let hex_str = format!("{:02x?}", block.initial_data);
                //draw_text(&format!("{}", hex_str), 100.0, 35.0, 18.0, BLACK);

                if is_mouse_button_down(MouseButton::Left) {
                    block.type_index.set(16);
                }
            },
            None => ()
        };

        let button_down = root_ui().button(Vec2::new(text_x_position, 500.0), "save to ./modified_chunk");

        if button_down {
            println!("save");

            let data = chunk.encode();
            fs::write("./modified_chunk", data).unwrap();
        };


        draw_rectangle(mouse_x, mouse_y, 10.0, 10.0, BLACK);

        next_frame().await;
    }
}

//#[macroquad::main("BasicShapes")]
//#[macroquad::main("Events")]

fn window_conf() -> Conf {
    Conf {
        window_title: "main.rs".to_owned(),
        fullscreen: false,
        window_width: (32 * 20) + 350,
        window_height: (32 * 20),//+ 40,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = &args[1];

    //let data = [0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x4b, 0x2a, 0xc8, 0xc9, 0x2c, 0x2e, 0x31, 0x30, 0x58, 0xca, 0xc8, 0xc4, 0xcc, 0xc2, 0xea, 0x31, 0x5f, 0x90, 0x01, 0x0c, 0x3c, 0x16, 0xc0, 0x18, 0x0b, 0x61, 0x8c, 0x45, 0x30, 0xc6, 0x62, 0x28, 0x83, 0x83, 0x4f, 0x5c, 0x41, 0xd3, 0x08, 0xc2, 0x66, 0x64, 0x84, 0xd0, 0x0c, 0x6c, 0x0c, 0xa8, 0xc0, 0x1a, 0x00, 0x14, 0xf5, 0x49, 0x74, 0x61, 0x00, 0x00, 0x00];
    //let mut decoder = Decoder::new(&data[..]).unwrap();
    //let mut buf = Vec::new();
    //decoder.read_to_end(&mut buf).unwrap();

    //let cursor = Cursor::new(buf);
    //let property_list = plist::Value::from_reader(cursor).unwrap();
    //let property_array = property_list.as_array().unwrap();

    //for data in property_array {
    //    println!("{:02x?}", data.as_data().unwrap());
    //}

    //println!("{:02x?}", buf);

    match mode.as_str() {
        "server" => {
            server::start(
                Ipv4Addr::new(0, 0, 0, 0),
                15151,
                ServerInformation {
                    world_name: String::from("server.rs"),
                    world_time: 3560.7666880898178,
                    welcome_message: String::from("connected yay!!!"),
                    start_portal_pos_x: 11695,
                    start_portal_pos_y: 743,
                    highest_point_x: 13970,
                    highest_point_y: 769,
                    credit: 0.0,
                    random_seed: 1710701067,
                    no_rain_timer: 886.6776123046875,
                    portal_level: 0,
                    save_id: String::from("9e9ab16b31c966ee30544355b0ab62ed"),
                    expert_mode: false,
                    minor_version: 3,
                    world_width_macro: 512
                }
            )
        },
        "client" => {
            client::start(
                Ipv4Addr::new(192, 168, 1, 11), 
                15151,
                ClientInformation {
                    alias: "client.rs".to_string(),
                    game_center_id: "6d1a8a3d4ac37d1dffa65b94a44839a9".to_string(),
                    minor_version: 3,
                    local: true,
                    udid_new: "593f1e5b13fc50d1713a97209da40d30".to_string(),
                    mic_or_speaker_on: true,
                    icloud_id: "4aa1b299fa81535707dcba8230a9b847".to_string(),
                    player_id: "d972d19b9bb03dc2429ce6337165aa85".to_string(),
                    voice_connected: false
                }
            )
        },
        "gui" => {
            gui().await;
        }
        _ => println!("invalid mode")
    }
}
