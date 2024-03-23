

use std::{env, fs};

use std::net::Ipv4Addr;
use std::str::FromStr;

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

use macroquad::prelude::*;
use macroquad::ui::*;
use macroquad::ui::widgets::Checkbox;
use macroquad::window::*;
use crate::handlers::block::Block;

const TILE_SIZE: i32 = 20;
const CHUNK_VIEWPORT_WIDTH: i32 = (TILE_SIZE * 32);
const CHUNK_VIEWPORT_HEIGHT: i32 = (TILE_SIZE * 32);
const DATA_VIEW_WIDTH: i32 = 400;
const DATA_VIEW_HEIGHT: i32 = 335;
const WINDOW_WIDTH: i32 = CHUNK_VIEWPORT_WIDTH + DATA_VIEW_WIDTH;
const WINDOW_HEIGHT: i32 = CHUNK_VIEWPORT_HEIGHT;
const GRID_THICKNESS: f32 = 0.4;

fn render_grid() {
    let mut render_color = BLACK;
    render_color.a = 0.6;

    let mut y = 0;
    while y < CHUNK_HEIGHT {
        let mut x = 0;
        while x < CHUNK_WIDTH {
            draw_rectangle((x as f32) * PIXEL_SIZE, 0.0, GRID_THICKNESS, 1000.0, render_color);
            x += 1
        }
        draw_rectangle(0.0, (y as f32) * PIXEL_SIZE, 1000.0, GRID_THICKNESS, render_color);
        y += 1
    }
}
fn render_chunk(chunk: &Chunk) {
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

            let pos = Vec2::new(f_x * PIXEL_SIZE, f_y * PIXEL_SIZE);

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
}
async fn render2() {
    let mut write_type_id = String::from("16");
    let mut write_subtype_id = String::from("0");

    let mut write_type_enabled = false;
    let mut write_subtype_enabled = false;

    let mut save_path = String::from("./modified_chunk");
    let mut open_chunk = Chunk::new();

    loop {
        let (mouse_x, mouse_y) = mouse_position();
        let hit_mouse_x = (mouse_x / PIXEL_SIZE).floor();
        let hit_mouse_y = (((mouse_y) / PIXEL_SIZE)).floor();

        clear_background(BLUE);

        root_ui().window(hash!(), Vec2::new(CHUNK_VIEWPORT_WIDTH as f32, 0.0), Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32), |ui| {

            widgets::Group::new(hash!(), Vec2::new(DATA_VIEW_WIDTH as f32, (DATA_VIEW_HEIGHT) as f32))
                .position(Vec2::new(0.0, 0.0))
                .ui(ui, |ui| {
                    ui.label(Vec2::new(315.0, 0.0), &format!("[x={hit_mouse_x},y={hit_mouse_y}]"));
                    let block_option = open_chunk.get_block(
                        hit_mouse_x as usize,
                        ((CHUNK_HEIGHT as f32) - hit_mouse_y - 1.0) as usize,
                    );

                    match block_option {
                        Some(block) => {
                            ui.label(Vec2::new(0.0, 0.0), &format!("Block: '{}'", block.get_name()));
                            ui.label(Vec2::new(0.0, 15.0), &format!("Background: '{}'", block.get_back_wall_name()));
                            ui.label(Vec2::new(0.0, 30.0), &format!("ZoneType: '{}'", block.zone_type_index.get()));
                            ui.label(Vec2::new(0.0, 45.0), &format!("Subtype: '{}'", block.get_subtype_name()));
                            ui.label(Vec2::new(0.0, 60.0), &format!("PartialContentLeft: '{}'", block.partial_contents_left.get()));
                            ui.label(Vec2::new(0.0, 75.0), &format!("GatherProgress: '{}'", block.gather_progress.get()));
                            ui.label(Vec2::new(0.0, 90.0), &format!("Light: '{}'", block.light.get()));
                            ui.label(Vec2::new(0.0, 105.0), &format!("SunLight: '{}'", block.sun_light.get()));
                            ui.label(Vec2::new(0.0, 120.0), &format!("SeasonOffset: '{}'", block.season_offset.get()));
                            ui.label(Vec2::new(0.0, 135.0), &format!("ExploredOffset: '{}'", block.explored_fraction.get()));
                            ui.label(Vec2::new(0.0, 150.0), &format!("TerrainSlowFactor: '{}'", block.terrain_slow_factor.get()));
                            ui.label(Vec2::new(0.0, 165.0), &format!("ForegroundContents: '{}'", block.foreground_contents.get()));
                            ui.label(Vec2::new(0.0, 180.0), &format!("BackgroundContents: '{}'", block.background_contents.get()));
                            ui.label(Vec2::new(0.0, 195.0), &format!("ArtificialLight: '({}, {}, {})'", block.artificial_light_r.get(), block.artificial_light_g.get(), block.artificial_light_b.get()));
                            ui.label(Vec2::new(0.0, 210.0), &format!("OnFire: '{}'", block.on_fire.get() != 0));
                            ui.label(Vec2::new(0.0, 225.0), &format!("DynamicObjectOwnerOld: '{}'", block.dynamic_object_owner_old.get()));
                            ui.label(Vec2::new(0.0, 240.0), &format!("PaintFront: '{}'", block.paint_front.get()));
                            ui.label(Vec2::new(0.0, 255.0), &format!("PaintTop: '{}'", block.paint_top.get()));
                            ui.label(Vec2::new(0.0, 270.0), &format!("PaintRight: '{}'", block.paint_right.get()));
                            ui.label(Vec2::new(0.0, 285.0), &format!("PaintLeft: '{}'", block.paint_left.get()));
                            ui.label(Vec2::new(0.0, 300.0), &format!("PaintBottom: '{}'", block.paint_bottom.get()));
                            ui.label(Vec2::new(0.0, 315.0), &format!("DynamicObjectOwner: '{}'", block.dynamic_object_owner.get()));
                        },
                        None => ()
                    }
                });
            widgets::Group::new(hash!(), Vec2::new(DATA_VIEW_WIDTH as f32, (CHUNK_VIEWPORT_HEIGHT - DATA_VIEW_HEIGHT) as f32))
                .position(Vec2::new(0.0, (DATA_VIEW_HEIGHT) as f32))
                .ui(ui, |ui| {
                    ui.label(None, "Placement settings");

                    let write_type_id_name = match u8::from_str(&write_type_id) {
                        Ok(id) => Block::get_name_from_type_id(id),
                        Err(_) => Block::get_name_from_type_id(0)
                    };
                    let write_subtype_id_name = match u8::from_str(&write_subtype_id) {
                        Ok(id) => Block::get_name_from_subtype(id),
                        Err(_) => Block::get_name_from_subtype(0)
                    };

                    ui.label(None, &format!("Type ID ({})", write_type_id_name));
                    ui.editbox(hash!(), Vec2::new(100.0, 20.0), &mut write_type_id);
                    ui.label(None, &format!("Subtype ID ({})", write_subtype_id_name));
                    ui.editbox(hash!(), Vec2::new(100.0, 20.0), &mut write_subtype_id);

                    widgets::Group::new(hash!(), Vec2::new((DATA_VIEW_WIDTH / 2) as f32, (DATA_VIEW_HEIGHT / 2) as f32))
                        .position(Vec2::new((DATA_VIEW_WIDTH / 2) as f32, 0.0))
                        .ui(ui, |ui| {
                            ui.label(None, "Path");
                            ui.editbox(hash!(), Vec2::new(150.0, 20.0), &mut save_path);
                            ui.checkbox(hash!(), &String::from("Type"), &mut write_type_enabled);
                            ui.checkbox(hash!(), &String::from("Subtype"), &mut write_subtype_enabled);
                            if ui.button(None, "Load") {
                                println!("Load!");
                                open_chunk = Chunk::decode(fs::read(&save_path).unwrap());
                            }
                            if ui.button(None, "Save") {
                                println!("Save!");
                                let chunk_data = open_chunk.encode();
                                fs::write(&save_path, chunk_data).unwrap();
                            }
                            /*if ui.button(None, "Save (new file)") {
                                println!("Save new file!");
                            }*/
                        });
                });
        });

        if is_mouse_button_down(MouseButton::Left) {
            match open_chunk.get_block(
                hit_mouse_x as usize,
                ((CHUNK_HEIGHT as f32) - hit_mouse_y - 1.0) as usize,
            ) {
                Some(block) => {
                    if write_type_enabled { block.type_index.set(u8::from_str(&write_type_id).unwrap()); };
                    if write_subtype_enabled { block.sub_type_index.set(u8::from_str(&write_subtype_id).unwrap()); };
                },
                None => ()
            };
        };

        render_chunk(&open_chunk);
        render_grid();

        next_frame().await;
    }
}

async fn gui() {
    let blocks = fs::read("./block_109_27_unmodified").unwrap();
    let chunk = Chunk::decode(blocks);

    loop {
        render2().await;
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
