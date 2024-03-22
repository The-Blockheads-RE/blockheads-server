use crate::handlers::dynamic_objects::{decode_generic_data, DynamicObject};

#[derive(std::fmt::Debug)]
pub struct Workbench {
    pub unique_id: i64,
    pub x: i32,
    pub y: i32,

    pub interaction_object_type: u8,
    pub object_type: u8,
    pub object_variant: u8
}

pub enum InteractionObjectType {
    Workbench = 1,
    Chest = 2,
    Bed = 3,
    Sign = 4,
    TradingPost = 5,
    TrainStation = 6,
    TradePortal = 7,
    OwnershipSign = 8,
    Mirror = 9
}

pub enum WorkbenchType {
    Undefined = 0,
    BasicPortal = 1,
    Workbench = 2,
    Campfire = 3,
    Weave = 4,
    Wood = 5,
    Tool = 6,
    Press = 7,
    Kiln = 8,
    Furnace = 9,
    Craft = 10,
    Mix = 11,
    Dye = 12,
    PlacedPortal = 13,
    Metalwork = 14,
    SteamGenerator = 15,
    ElectricKiln = 16,
    ElectricFurnace = 17,
    ElectricMetalworkBench = 18,
    ElectricStove = 19,
    SolarPanel = 20,
    Flywheel = 21,
    ArmorBench = 22,
    TrainYard = 23,
    Easel = 24,
    Build = 25,
    Refinery = 26,
    ElectricPress = 27,
    CompostBin = 28,
    Sluice = 29,
    EggExtractor = 30,
    PizzaOven = 31
}

impl Workbench {
    pub fn decode(raw_data: Vec<u8>) -> Self {
        let specific_data = raw_data[32..].to_vec();
        let generic_data = decode_generic_data(raw_data);

        return Workbench {
            unique_id: generic_data.unique_id,
            x: generic_data.x,
            y: generic_data.y,

            interaction_object_type: specific_data[32],
            object_type: specific_data[50],
            object_variant: specific_data[51]
        };
    }
}

impl DynamicObject for Workbench {
    fn get_unique_id(&self) -> i64 { return self.unique_id; }
    fn get_x(&self) -> i32 {
        return self.x;
    }
    fn get_y(&self) -> i32 {
        return self.y
    }
    fn encode_obj(&self) -> Vec<u8> {
        let mut encoded_data = Vec::new();

        encoded_data.insert(0, self.interaction_object_type);
        let mut unknown_padding = [
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0
        ].to_vec();

        encoded_data.append(&mut unknown_padding);

        encoded_data.insert(18, self.object_type);
        encoded_data.insert(19, self.object_variant);
        let mut unknown_padding_2 = [
            0, 0, 0, 0
        ].to_vec();

        encoded_data.append(&mut unknown_padding_2);

        return encoded_data;
    }
    fn get_type_id(&self) -> u8 { return 45; }
}