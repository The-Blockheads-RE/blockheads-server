#[derive(Debug)]
pub struct Block {
    pub type_index: Cell<u8>,
    pub back_wall_type_index: Cell<u8>,
    pub sub_type_index: Cell<u8>,
    pub zone_type_index: Cell<u8>,
    pub partial_contents_left: Cell<u8>,
    pub gather_progress: Cell<u8>,
    pub light: Cell<u8>,
    pub sun_light: Cell<u8>,
    pub season_offset: Cell<u8>,
    pub explored_fraction: Cell<u8>,
    pub terrain_slow_factor: Cell<u8>,
    pub foreground_contents: Cell<u8>,
    pub background_contents: Cell<u8>,

    pub initial_data: Vec<u8>
}

use std::cell::Cell;
use std::fmt;
use std::fmt::Debug;

use macroquad::prelude::*;

pub const BLOCK_SIZE: usize = 64;

impl Block {
    pub fn get_name_from_subtype(subtype_id: u8) -> String {
        let str = match subtype_id {
            2 => "CLAY_ORE",
            6 => "LEAVES",
            8 => "WOOD",
            46 => "WORKBENCH",
            47 => "PORTAL_GATE",
            61 => "COPPER_ORE",
            62 => "TIN_ORE",
            63 => "IRON_ORE",
            64 => "OIL",
            65 => "COAL",
            77 => "GOLD_NUGGETS",
            106 => "PLATINUM_ORE",
            107 => "TITANIUM_ORE",
            _ => {
                let s = format!("? ({})", subtype_id);
                return s;
            }
        };

        return str.to_owned()
    }
    pub fn get_color_from_type_id(type_id: u8) -> Color {
        let name = Block::get_name_from_type_id(type_id);

        let res = match name.as_str() {
            "Rock" => Color::from_rgba(96, 96, 96, 255),
            "Air" => Color::from_rgba(255, 255, 255, 0),
            //
            //
            "Snow" => Color::from_rgba(255, 255, 255, 255),
            "Dirt" => Color::from_rgba(128, 67, 37, 255),
            "Dirt Grass Frozen" => Color::from_rgba(200, 200, 200, 255),
            "Dirt Grass" => Color::from_rgba(81, 196, 75, 255),
            "Initial Portal Base Stone" => Color::from_rgba(0, 0, 255, 255),
            "Time Crystal" => Color::from_rgba(0, 247, 255, 255),
            _ => {
                println!("unhandled {}", name);
                return Color::from_rgba(236, 3, 252, 255);
            }
        };

        return res
    }
    pub fn get_name_from_type_id(type_id: u8) -> String {
        let str = match type_id {
            1 => "Rock",
            2 => "Air",
            3 => "Water",
            4 => "Ice",
            5 => "Snow",
            6 => "Dirt",
            7 => "Sand",
            8 => "Sand Beach",
            9 => "Wood",
            10 => "Cobblestone",
            11 => "Red Bricks",
            12 => "Limestone",
            13 => "Limestone Block",
            14 => "Marble",
            15 => "Marble Block",
            16 => "Time Crystal",
            17 => "Sandstone",
            18 => "Sandstone Block",
            19 => "Red Marble",
            20 => "Red Marble Block",
            21 => "Fax Mat",
            22 => "Flax Mat Yellow",
            23 => "Flax Mat Red",
            24 => "Glass",
            25 => "Initial Portal Base Stone",
            26 => "Gold Block",
            27 => "Dirt Grass",
            28 => "Dirt Grass Frozen",
            29 => "Lapis",
            30 => "Lapis Block",
            31 => "Lava",
            32 => "Wooden Platform",
            33 => "Initial Portal Base Amethyst",
            34 => "Initial Portal Base Sapphire",
            35 => "Initial Portal Base Ruby",
            36 => "Initial Portal Base Diamond",
            37 => "North Pole",
            38 => "South Pole",
            39 => "West Pole",
            40 => "East Pole",
            41 => "Placed Portal Base Stone",
            42 => "Placed Portal Base Amethyst",
            43 => "Placed Portal Base Sapphire",
            44 => "Placed Portal Base Ruby",
            45 => "Placed Portal Base Diamond",
            46 => "Compost",
            47 => "Compost Grass",
            48 => "Compost Grass Frozen",
            49 => "Bassalt",
            50 => "Bassalt Block",
            51 => "Copper Block",
            52 => "Tin Block",
            53 => "Bronze Block",
            54 => "Iron Block",
            55 => "Steel Block",
            56 => "Black Sand",
            57 => "Black Glass",
            58 => "Trade Portal Base Stone",
            59 => "Trade Portal Base Amethyst",
            60 => "Trade Portal Base Sapphire",
            61 => "Trade Portal Base Ruby",
            62 => "Trade Portal Base Diamond",
            63 => "Fallen Leaves",
            64 => "Platinum Block",
            65 => "Titanium Block",
            66 => "Carbon Fiber Block",
            67 => "Gravel",
            68 => "Amethyst Block",
            69 => "Sapphire Block",
            70 => "Emerald Block",
            71 => "Ruby Block",
            72 => "Diamond Block",
            73 => "Plaster",
            74 => "Luminous Plaster",
            75 => "Max Tile Types",
            _ => "Unknown Index"
        };

        return str.to_string();
    }
}

impl Block {
    pub fn encode(&self) -> Vec<u8> {
        let mut encoded_block = [0; 64].to_vec();

        encoded_block[0] = self.type_index.get();
        encoded_block[1] = self.back_wall_type_index.get();
        encoded_block[2] = self.zone_type_index.get();
        encoded_block[3] = self.sub_type_index.get();
        encoded_block[4] = self.partial_contents_left.get();
        encoded_block[5] = self.gather_progress.get();
        encoded_block[6] = self.light.get();
        encoded_block[7] = self.sun_light.get();
        encoded_block[8] = self.season_offset.get();
        encoded_block[9] = self.explored_fraction.get();
        encoded_block[10] = self.terrain_slow_factor.get();
        encoded_block[11] = self.foreground_contents.get();
        encoded_block[12] = self.background_contents.get();

        return encoded_block;
    }
    pub fn decode(raw_data: Vec<u8>) -> Self {
        return Self {
            type_index: Cell::new(raw_data[0]),
            back_wall_type_index: Cell::new(raw_data[1]),
            zone_type_index: Cell::new(raw_data[2]),
            sub_type_index: Cell::new(raw_data[3]),
            partial_contents_left: Cell::new(raw_data[4]),
            gather_progress: Cell::new(raw_data[5]),   
            light: Cell::new(raw_data[6]),
            sun_light: Cell::new(raw_data[7]),
            season_offset: Cell::new(raw_data[8]),
            explored_fraction: Cell::new(raw_data[9]),
            terrain_slow_factor: Cell::new(raw_data[10]),
            foreground_contents: Cell::new(raw_data[11]),
            background_contents: Cell::new(raw_data[11]),
            initial_data: raw_data
        }
    }
    //pub fn new() -> Self {
    //    return Self {
    //        type_index: Cell::new(2), // air
    //        back_wall_type_index: 2,
    //        sub_type_index: 0,
    //        gather_progress: 0,
    //        partial_contents_left: 0,
    //        foreground_contents: 0,
    //        initial_data: [0; BLOCK_SIZE].to_vec()
    //    }
    //}
}

impl Block {
    pub fn get_name(&self) -> String {
        return Block::get_name_from_type_id(self.type_index.get());
    }
    pub fn get_back_wall_name(&self) -> String {
        return Block::get_name_from_type_id(self.back_wall_type_index.get());
    }
    pub fn get_color(&self) -> Color {
        return Block::get_color_from_type_id(self.type_index.get());
    }
    pub fn get_back_wall_color(&self) -> Color {
        return Block::get_color_from_type_id(self.back_wall_type_index.get());
    }
    pub fn get_subtype_name(&self) -> String {
        return Block::get_name_from_subtype(self.sub_type_index.get())
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.type_index.get())
    }
}