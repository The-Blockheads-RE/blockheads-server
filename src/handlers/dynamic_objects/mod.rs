use std::io::Cursor;
use plist::Value;
use crate::handlers::Compression::GZIP;
use crate::handlers::dynamic_objects::free_block::FreeBlock;

pub struct DynamicObjectHandler {
    pub objects: Vec<Box<dyn DynamicObject>>
}

pub unsafe fn unbox_dynamic_object<T>(boxed_dynamic_object: Box<T>) -> T {
    return *boxed_dynamic_object;
}

pub trait DynamicObject {
    fn get_unique_id(&self) -> i64;
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;

    fn encode(&self) -> Vec<u8> {
        let mut data = Vec::new();

        let mut generic_dynamic_data = self.encode_generic_dynamic_data();
        let mut obj_data = self.encode_obj();

        data.append(&mut generic_dynamic_data);
        data.append(&mut obj_data);

        return data;
    }
    fn encode_obj(&self) -> Vec<u8>;
    fn encode_generic_dynamic_data(&self) -> Vec<u8> {
        let mut generic_data = Vec::new();

        let mut unique_id = self.get_unique_id().to_le_bytes().to_vec();
        let mut x_bytes = self.get_x().to_le_bytes().to_vec();
        let mut y_bytes = self.get_y().to_le_bytes().to_vec();
        let mut unknown = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec();
        //let mut unknown = [0, 66, 80, 0, 31, 147, 107, 98, 0, 0, 0, 0, 0, 0, 0, 0].to_vec();

        generic_data.append(&mut unique_id);
        generic_data.append(&mut x_bytes);
        generic_data.append(&mut y_bytes);
        generic_data.append(&mut unknown);

        println!("generic: {:02x?}", generic_data);
        
        return generic_data.to_vec();
    }
    fn get_type_id(&self) -> u8;
    fn encode_and_compress(&self) -> Vec<u8> {
        let encoded_data = self.encode();

        let data_uncompressed_data = Value::Data(encoded_data);
        let mut array = Vec::new();
        array.insert(0, data_uncompressed_data);
        let p_list = plist::Value::Array(array);
        let mut cursor = Cursor::new(Vec::new());
        p_list.to_writer_binary(&mut cursor).unwrap();
        let data = cursor.into_inner();

        let mut compressed_data = GZIP::encode(&data).unwrap();
        compressed_data.insert(0, self.get_type_id());

        return compressed_data;
    }
}

struct GenericDynamicObjectData {
    unique_id: i64,
    x: i32,
    y: i32,
}

pub fn decode_generic_data(raw_data: Vec<u8>) -> GenericDynamicObjectData {
    return GenericDynamicObjectData {
        unique_id: i64::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3], raw_data[4], raw_data[5], raw_data[6], raw_data[7]]),
        x: i32::from_le_bytes([raw_data[8], raw_data[9], raw_data[10], raw_data[11]]),
        y: i32::from_le_bytes([raw_data[12], raw_data[13], raw_data[14], raw_data[15]])
    };
}

pub struct UnknownDynamicObject {
    unique_id: i64,
    x: i32,
    y: i32
}
impl DynamicObject for UnknownDynamicObject {
    fn get_unique_id(&self) -> i64 {
        return self.unique_id;
    }
    fn get_x(&self) -> i32 {
        return self.x
    }
    fn get_y(&self) -> i32 {
        return self.y
    }
    fn encode_obj(&self) -> Vec<u8> {
        return Vec::new();
    }
    fn get_type_id(&self) -> u8 {
        return 0;
    }
}
impl UnknownDynamicObject {
    pub fn decode(raw_data: Vec<u8>) -> impl DynamicObject {
        let generic_data = decode_generic_data(raw_data);
        return UnknownDynamicObject {
            unique_id: generic_data.unique_id,
            x: generic_data.x,
            y: generic_data.y
        };
    }
}

pub mod free_block;
pub mod workbench;

impl DynamicObjectHandler {
    pub fn decode(type_id: u8, raw_data: Vec<u8>) -> Box<dyn DynamicObject> {
        return match type_id {
            14 => Box::new(FreeBlock::decode(raw_data)),
            _ => Box::new(UnknownDynamicObject::decode(raw_data)) // placeholder
        };
    }
    pub fn get_name_from_id(type_id: u8) -> String {
        match type_id {
            1 => "APPLE_TREE", // 0x1
            2 => "MAPLE_TREE", // 0x2
            3 => "MANGO_TREE", // 0x3
            4 => "PINE_TREE", // 0x4
            5 => "CACTUS_TREE", // 0x5
            6 => "COCONUT_TREE", // 0x6
            7 => "ORANGE_TREE", // 0x7
            8 => "CHERRY_TREE", // 0x8
            9 => "COFFEE_TREE", // 0x9
            10 => "FLAX_PLANT", // 0xA
            11 => "SUNFLOWER_PLANT", // 0xB
            12 => "CORN_PLANT", // 0xC
            13 => "NPC_DODO", // 0xD
            14 => "FREE_BLOCK", // 0xE
            15 => "INTERACTION_TYPE_DEPRECATED", // 0xF
            16 => "FIRE", // 0x10
            17 => "TORCH", // 0x11
            18 => "GLOWBLOCK", // 0x12
            19 => "LADDER", // 0x13
            20 => "DOOR", // 0x14
            21 => "ARTIFICIAL_LIGHT", // 0x15
            22 => "SURFACEBLOCK", // 0x16
            23 => "BED", // 0x17
            24 => "BLOCKHEAD", // 0x18
            25 => "NPC_DROPBEAR", // 0x19
            26 => "GATHERBLOCK", // 0x1A
            27 => "CARROT_PLANT", // 0x1B
            28 => "NPC_DONKEY", // 0x1C
            29 => "SNOW_SURFACEBLOCK", // 0x1D
            30 => "EGG", // 0x1E
            31 => "WINDOW", // 0x1F
            32 => "BOAT", // 0x20
            33 => "CHILLI_PLANT", // 0x21
            34 => "KELP_PLANT", // 0x22
            35 => "NPC_CLOWNFISH", // 0x23
            36 => "NPC_SHARK", // 0x24
            37 => "LIME_TREE", // 0x25
            38 => "WIRE", // 0x26
            39 => "NPC_CAVETROLL", // 0x27
            40 => "RAIL", // 0x28
            41 => "TRAIN_CAR_HANDCAR", // 0x29
            42 => "TRAIN_CAR_STEAMENGINE", // 0x2A
            43 => "TRAIN_CAR_FREIGHT", // 0x2B
            44 => "TRAIN_CAR_PASSENGER", // 0x2C
            45 => "WORKBENCH", // 0x2D
            46 => "CHEST", // 0x2E
            47 => "SIGN", // 0x2F
            48 => "TRADING_POST", // 0x30
            49 => "TRAIN_STATION", // x031
            50 => "TRADE_PORTAL", // 0x32
            51 => "NPC_SCORPION", // 0x33
            52 => "PAINTING", // 0x34
            53 => "COLUMN", // 0x35
            54 => "STAIRS", // 0x36
            55 => "ELEVATOR_MOTOR", // 0x37
            56 => "ELEVATOR_SHAFT", // 0x38
            57 => "GEM_TREE", // 0x39
            58 => "VINE_PLANT", // 0x3A
            59 => "TULIP_PLANT", // 0x3B
            60 => "OWNERSHIP_SIGN", // 0x3C
            61 => "WHEAT_PLANT", // 0x3D
            62 => "TOMATO_PLANT", // 0x3E
            63 => "NPC_YAK", // 0x3F
            64 => "MIRROR", // 0x40
            _ => "Invalid"
        }.to_owned()
    }
}