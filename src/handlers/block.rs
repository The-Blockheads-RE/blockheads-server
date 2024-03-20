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
            1 => "FLINT",
            2 => "CLAY",
            3 => "APPLE_TREE_BUSH",
            4 => "APPLE_TREE_TRUNK",
            5 => "APPLE_TREE_TRUNK_BUSH",
            6 => "PINE_TREE_BUSH",
            7 => "PINE_TREE_TRUNK",
            8 => "PINE_TREE_TRUNK_BUSH",
            9 => "MAPLE_TREE_BUSH",
            10 => "MAPLE_TREE_TRUNK",
            11 => "MAPLE_TREE_TRUNK_BUSH",
            12 => "MANGO_TREE_BUSH",
            13 => "MANGO_TREE_TRUNK",
            14 => "MANGO_TREE_TRUNK_BUSH",
            15 => "COCONUT_TREE_BUSH",
            16 => "COCONUT_TREE_TRUNK",
            17 => "COCONUT_TREE_TRUNK_BUSH",
            18 => "ORANGE_TREE_BUSH",
            19 => "ORANGE_TREE_TRUNK",
            20 => "ORANGE_TREE_TRUNK_BUSH",
            21 => "CHERRY_TREE_BUSH",
            22 => "CHERRY_TREE_TRUNK",
            23 => "CHERRY_TREE_TRUNK_BUSH",
            24 => "COFFEE_TREE_BUSH",
            25 => "COFFEE_TREE_TRUNK",
            26 => "COFFEE_TREE_TRUNK_BUSH",
            27 => "APPLE_TREE_BUSH_DEAD",
            28 => "APPLE_TREE_TRUNK_DEAD",
            29 => "PINE_TREE_TRUNK_DEAD",
            30 => "MAPLE_TREE_BUSH_DEAD",
            31 => "MAPLE_TREE_TRUNK_DEAD",
            32 => "MANGO_TREE_BUSH_DEAD",
            33 => "MANGO_TREE_TRUNK_DEAD",
            34 => "PINE_TREE_BUSH_DEAD",
            35 => "COCONUT_TREE_BUSH_DEAD",
            36 => "COCONUT_TREE_TRUNK_DEAD",
            37 => "ORANGE_TREE_BUSH_DEAD",
            38 => "ORANGE_TREE_TRUNK_DEAD",
            39 => "CHERRY_TREE_BUSH_DEAD",
            40 => "CHERRY_TREE_TRUNK_DEAD",
            41 => "COFFEE_TREE_BUSH_DEAD",
            42 => "COFFEE_TREE_TRUNK_DEAD",
            43 => "CACTUS",
            44 => "CACTUS_DEAD",
            45 => "CHARCOAL",
            46 => "WORKBENCH",
            47 => "WORKBENCH_SPRITE",
            48 => "INTERACTION_OBJECT",
            49 => "BASIC_TORCH_SPRITE",
            50 => "CLAY_LANTERN_SPRITE",
            51 => "RUBY",
            52 => "RUBY_PROMISE",
            53 => "EMERALD",
            54 => "EMERALD_PROMISE",
            55 => "SAPPHIRE",
            56 => "SAPPHIRE_PROMISE",
            57 => "AMETHYST",
            58 => "AMETHYST_PROMISE",
            59 => "DIAMOND",
            60 => "DIAMOND_PROMISE",
            61 => "COPPER",
            62 => "TIN",
            63 => "IRON",
            64 => "OIL",
            65 => "COAL",
            66 => "LADDER",
            67 => "FLAX_PLANT",
            68 => "FLAX_FLOWER",
            69 => "WINDOW",
            70 => "DOOR",
            71 => "SUNFLOWER_PLANT",
            72 => "SUNFLOWER_FLOWER",
            73 => "CORN_PLANT",
            74 => "CORN_FLOWER",
            75 => "TRAPDOOR",
            76 => "CARROT_PLANT",
            77 => "GOLD",
            78 => "DODO_EGG",
            79 => "CHILLI_PLANT",
            80 => "CHILLI_FLOWER",
            81 => "KELP_PLANT",
            82 => "AMETHYST_CHANDELIER_SPRITE",
            83 => "SAPPHIRE_CHANDELIER_SPRITE",
            84 => "EMERALD_CHANDELIER_SPRITE",
            85 => "RUBY_CHANDELIER_SPRITE",
            86 => "DIAMOND_CHANDELIER_SPRITE",
            87 => "STEEL_LANTERN_SPRITE",
            88 => "SHARK_JAW",
            89 => "LIME_TREE_BUSH",
            90 => "LIME_TREE_TRUNK",
            91 => "LIME_TREE_TRUNK_BUSH",
            92 => "LIME_TREE_BUSH_DEAD",
            93 => "LIME_TREE_TRUNK_DEAD",
            94 => "TREASURE_CHEST_PROMISE",
            95 => "BLACK_WINDOW",
            96 => "COPPER_WIRE",
            97 => "ICE_TORCH_SPRITE",
            98 => "RAIL",
            99 => "PAINTING",
            100 => "COLUMN",
            101 => "STAIRS",
            102 => "STEEL_DOWNLIGHT",
            103 => "ELEVATOR_SHAFT",
            104 => "ELEVATOR_MOTOR",
            105 => "STEEL_UPLIGHT",
            106 => "PLATINUM",
            107 => "TITANIUM",
            108 => "AMETHYST_TREE_PROMISE",
            109 => "AMEHTYST_TREE_TRUNK",
            110 => "AMETHYST_TREE_BUSH",
            111 => "AMETHYST_TREE_TRUNK_BUSH",
            112 => "SAPPHIRE_TREE_TRUNK",
            113 => "SAPPHIRE_TREE_BUSH",
            114 => "SAPPHIRE_TREE_TRUNK_BUSH",
            115 => "EMERALD_TREE_TRUNK",
            116 => "EMERALD_TREE_BUSH",
            117 => "EMERALD_TREE_TRUNK_BUSH",
            118 => "RUBY_TREE_TRUNK",
            119 => "RUBY_TREE_BUSH",
            120 => "RUBY_TREE_TRUNK_BUSH",
            121 => "DIAMOND_TREE_TRUNK",
            122 => "DIAMOND_TREE_BUSH",
            123 => "DIAMOND_TREE_TRUNK_BUSH",
            124 => "VINE",
            125 => "TULIP_PLANT",
            126 => "TULIP_PLANT_PROMISE_PLAIN",
            127 => "VINE_PROMISE",
            128 => "SAPPHIRE_TREE_PROMISE",
            129 => "EMERALD_TREE_PROMISE",
            130 => "RUBY_TREE_PROMISE",
            131 => "DIAMOND_TREE_PROMISE",
            132 => "APPLE_TREE_PROMISE",
            133 => "ORANGE_TREE_PROMISE",
            134 => "CHERRY_TREE_PROMISE",
            135 => "PINE_TREE_PROMISE",
            136 => "CACTUS_TREE_PROMISE",
            137 => "LIME_TREE_PROMISE",
            138 => "COFFEE_TREE_PROMISE",
            139 => "COCONUT_TREE_PROMISE",
            140 => "MAPLE_TREE_PROMISE",
            141 => "MANGO_TREE_PROMISE",
            142 => "TULIP_PLANT_PROMISE_RARE",
            143 => "TULIP_PLANT_PROMISE_RAREST",
            144 => "CAVE_TROLL_PROMISE_NO_TREASURE",
            145 => "TREASURE_CHEST_PROMISE_NO_CAVE_TROLL",
            146 => "GATE",
            147 => "WHEAT_PLANT",
            148 => "TOMATO_PLANT",
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
    pub fn new() -> Self { // Creates an empty block
        return Self {
            type_index: Cell::new(0), // air
            back_wall_type_index: Cell::new(0),
            zone_type_index: Cell::new(1),
            sub_type_index: Cell::new(0),
            partial_contents_left: Cell::new(0),
            gather_progress: Cell::new(0),
            light: Cell::new(0),
            sun_light: Cell::new(0),
            season_offset: Cell::new(0),
            explored_fraction: Cell::new(0),
            terrain_slow_factor: Cell::new(0),
            foreground_contents: Cell::new(0),
            background_contents: Cell::new(0),
            initial_data: [0; BLOCK_SIZE].to_vec(),
        }
    }
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