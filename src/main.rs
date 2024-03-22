

use std::{env, fs};

use std::net::Ipv4Addr;

pub mod handlers;

use handlers::{chunk::*};
use handlers::server;
use handlers::ServerInformation::ServerInformation;
use handlers::client;
use handlers::client::ClientInformation;
use handlers::dynamic_objects::DynamicObjectHandler;

use macroquad::{prelude::*};
use macroquad::ui::{root_ui};
use crate::handlers::dynamic_objects::free_block::{FreeBlock, ItemType};
use crate::handlers::dynamic_objects::{DynamicObject, unbox_dynamic_object};

const PIXEL_SIZE: f32 = 20.0;
const TEXT_X_POSITION: f32 = (CHUNK_WIDTH as f32) * PIXEL_SIZE;

fn window_conf() -> Conf {
    Conf {
        window_title: "main.rs".to_owned(),
        fullscreen: false,
        window_width: (32 * 20) + 450,
        window_height: (32 * 20),//+ 40,
        window_resizable: false,
        ..Default::default()
    }
}

async fn render(chunk: &Chunk) {
    clear_background(BLUE);

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

            let pos = Vec2::new(f_x * PIXEL_SIZE, (f_y * PIXEL_SIZE) + 0.0);

            draw_rectangle(pos.x, pos.y, PIXEL_SIZE, PIXEL_SIZE, background_color);
            draw_rectangle(pos.x, pos.y, PIXEL_SIZE, PIXEL_SIZE, block.get_color());

            if block.sub_type_index.get() != 0 {
                draw_line(pos.x, pos.y, pos.x + PIXEL_SIZE, pos.y + PIXEL_SIZE, 3.0, PURPLE);
                draw_line(pos.x + PIXEL_SIZE, pos.y, pos.x, pos.y + PIXEL_SIZE, 3.0, PURPLE);
            }
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

            draw_text(&format!("mouse {}, {}", hit_mouse_x, hit_mouse_y), TEXT_X_POSITION, 20.0, font_size, BLACK);
            draw_text(&format!("block '{}'", block.get_name()), TEXT_X_POSITION, 40.0, font_size, color);
            draw_text(&format!("background '{}'", block.get_back_wall_name()), TEXT_X_POSITION, 60.0, font_size, background_color);
            draw_text(&format!("zone_type '{}'", block.zone_type_index.get()), TEXT_X_POSITION, 80.0, font_size, BLACK);
            draw_text(&format!("subtype '{}'", block.get_subtype_name()), TEXT_X_POSITION, 100.0, font_size, BLACK);
            draw_text(&format!("partial_content_left '{}'", block.partial_contents_left.get()), TEXT_X_POSITION, 120.0, font_size, BLACK);
            draw_text(&format!("damage '{}'", block.gather_progress.get()), TEXT_X_POSITION, 140.0, font_size, BLACK);
            draw_text(&format!("light '{}'", block.light.get()), TEXT_X_POSITION, 160.0, font_size, BLACK);
            draw_text(&format!("sun_light '{}'", block.sun_light.get()), TEXT_X_POSITION, 180.0, font_size, BLACK);
            draw_text(&format!("season_offset '{}'", block.season_offset.get()), TEXT_X_POSITION, 200.0, font_size, BLACK);
            draw_text(&format!("explored_fraction '{}'", block.explored_fraction.get()), TEXT_X_POSITION, 220.0, font_size, BLACK);
            draw_text(&format!("terrain_slow_factor '{}'", block.terrain_slow_factor.get()), TEXT_X_POSITION, 240.0, font_size, BLACK);
            draw_text(&format!("foreground_contents '{}'", block.foreground_contents.get()), TEXT_X_POSITION, 260.0, font_size, BLACK);
            draw_text(&format!("background_contents '{}'", block.background_contents.get()), TEXT_X_POSITION, 280.0, font_size, BLACK);
            draw_text(&format!("artificial_light_r '{}'", block.artificial_light_r.get()), TEXT_X_POSITION, 300.0, font_size, BLACK);
            draw_text(&format!("artificial_light_g '{}'", block.artificial_light_g.get()), TEXT_X_POSITION, 320.0, font_size, BLACK);
            draw_text(&format!("artificial_light_b '{}'", block.artificial_light_b.get()), TEXT_X_POSITION, 340.0, font_size, BLACK);
            draw_text(&format!("artificial_heat '{}'", block.artificial_heat.get()), TEXT_X_POSITION, 360.0, font_size, BLACK);
            draw_text(&format!("on_fire '{}'", block.on_fire.get()), TEXT_X_POSITION, 380.0, font_size, BLACK);
            draw_text(&format!("dynamic_object_owner_old '{}'", block.dynamic_object_owner_old.get()), TEXT_X_POSITION, 400.0, font_size, BLACK);
            draw_text(&format!("paint_front '{}'", block.paint_front.get()), TEXT_X_POSITION, 420.0, font_size, BLACK);
            draw_text(&format!("paint_top '{}'", block.paint_top.get()), TEXT_X_POSITION, 440.0, font_size, BLACK);
            draw_text(&format!("paint_right '{}'", block.paint_right.get()), TEXT_X_POSITION, 460.0, font_size, BLACK);
            draw_text(&format!("paint_left '{}'", block.paint_left.get()), TEXT_X_POSITION, 480.0, font_size, BLACK);
            draw_text(&format!("paint_bottom '{}'", block.paint_bottom.get()), TEXT_X_POSITION, 500.0, font_size, BLACK);
            draw_text(&format!("dynamic_object_owner '{}'", block.dynamic_object_owner.get()), TEXT_X_POSITION, 520.0, font_size, BLACK);

            if is_mouse_button_down(MouseButton::Left) {
                //block.type_index.set(16);
                block.sub_type_index.set(46);
            }
        },
        None => ()
    };

    let button_down = root_ui().button(Vec2::new(TEXT_X_POSITION, 530.0), "save to ./modified_chunk");

    if button_down {
        println!("save");

        let data = chunk.encode();
        fs::write("./block_109_27", data).unwrap();
    };

    draw_rectangle(mouse_x, mouse_y, 10.0, 10.0, BLACK);
}

async fn gui() {
    let blocks = fs::read("./block_109_27_unmodified").unwrap();
    let chunk = Chunk::decode(blocks);
    chunk.print();

    //root_ui().input_text(90, "input", &mut inp);

    loop {
        render(&chunk).await;
        next_frame().await;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let raw_data = [0xe1, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb0, 0x2d, 0x00, 0x00, 0xed, 0x02, 0x00, 0x00, 0x00, 0x9a, 0x2c, 0x8f, 0x01, 0x93, 0x6b, 0x62, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf4, 0xff, 0x00, 0x00, 0x7f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00].to_vec();

    let args: Vec<String> = env::args().collect();

    let mode = match args.get(1) {
        Some(mode) => mode,
        None => {
            println!("Mode argument should be supplied! (gui/client/server)");
            std::process::exit(1);
        }
    };

    match mode.as_str() {
        "server" => {
            server::start(
                Ipv4Addr::new(0, 0, 0, 0),
                15151,
                ServerInformation {
                    world_name: String::from("server.rs"),
                    world_time: 3560.7666880898178,
                    welcome_message: Some(String::from("connected yay!!!")),
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
        _ => {
            println!("Invalid mode argument suppled! Should be: (gui/client/server)");
            std::process::exit(1);
        }
    }
}
