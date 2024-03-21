pub struct DynamicObject {

}

impl DynamicObject {
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