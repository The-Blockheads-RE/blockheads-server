use std::io::{Cursor, Read, Write};
use libflate::gzip::{Decoder, Encoder};
use plist::{Integer, Value};

pub struct ServerInformation {
    pub world_name: String,
    pub world_time: f64,
    pub welcome_message: Option<String>,
    pub start_portal_pos_x: i64,
    pub start_portal_pos_y: i64,
    pub highest_point_x: i64,
    pub highest_point_y: i64,
    pub credit: f64,
    pub random_seed: i64,
    pub no_rain_timer: f64,
    pub portal_level: i64,
    pub save_id: String,
    pub expert_mode: bool,
    pub minor_version: i64,
    pub world_width_macro: i64
}

impl ServerInformation {
    pub fn encode(&self) -> Vec<u8> {
        let mut server_information_plist = plist::Dictionary::new();

        server_information_plist.insert(
            String::from("worldName"),
            Value::String(self.world_name.clone())
        );
        server_information_plist.insert(
            String::from("worldTime"),
            Value::Real(self.world_time)
        );
        match &self.welcome_message {
            Some(welcome_message) => {
                server_information_plist.insert(
                    String::from("welcomeMessage"),
                    Value::String(welcome_message.clone())
                );
            }
            None => ()
        };

        server_information_plist.insert(
            String::from("startPortalPos.x"),
            Value::Integer(Integer::from(self.start_portal_pos_x))
        );
        server_information_plist.insert(
            String::from("startPortalPos.y"),
            Value::Integer(Integer::from(self.start_portal_pos_y))
        );
        server_information_plist.insert(
            String::from("highestPoint.x"),
            Value::Integer(Integer::from(self.highest_point_x))
        );
        server_information_plist.insert(
            String::from("highestPoint.y"),
            Value::Integer(Integer::from(self.highest_point_y))
        );
        server_information_plist.insert(
            String::from("credit"),
            Value::Real(self.credit)
        );
        server_information_plist.insert(
            String::from("randomSeed"),
            Value::Integer(Integer::from(self.random_seed))
        );
        server_information_plist.insert(
            String::from("noRainTimer"),
            Value::Real(self.no_rain_timer)
        );
        server_information_plist.insert(
            String::from("portalLevel"),
            Value::Integer(Integer::from(self.portal_level))
        );
        server_information_plist.insert(
            String::from("saveID"),
            Value::String(self.save_id.clone())
        );
        server_information_plist.insert(
            String::from("expertMode"),
            Value::Boolean(self.expert_mode)
        );
        server_information_plist.insert(
            String::from("minorVersion"),
            Value::Integer(Integer::from(self.minor_version))
        );
        server_information_plist.insert(
            String::from("worldWidthMacro"),
            Value::Integer(Integer::from(self.world_width_macro))
        );

        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        plist::to_writer_binary(&mut cursor, &server_information_plist).unwrap();
        plist::to_file_xml("./tried_serverinfo", &server_information_plist).unwrap();
        let uncompressed_data = cursor.into_inner();

        let mut encoder: Encoder<Vec<u8>> = Encoder::new(Vec::new()).unwrap();
        encoder.write_all(&uncompressed_data).unwrap();

        let mut gzipped_data = encoder.finish().into_result().unwrap();

        return gzipped_data;
    }
    pub fn decode(raw_data: Vec<u8>) -> Self {
        let mut decoder = Decoder::new(&raw_data[..]).unwrap();
        let mut server_info_buffer = Vec::new();
        decoder.read_to_end(&mut server_info_buffer).unwrap();

        let cursor = Cursor::new(server_info_buffer);
        let parsed_plist = plist::Value::from_reader(cursor).unwrap();
        let dict = parsed_plist.as_dictionary().unwrap();

        return Self {
            world_name: dict.get("worldName").unwrap().as_string().unwrap().to_owned(),
            world_time: dict.get("worldTime").unwrap().as_real().unwrap(),
            welcome_message: match dict.get("welcomeMessage") {
                Some(welcome_message_value) => Some(welcome_message_value.as_string().unwrap().to_owned()),
                None => None
            },
            start_portal_pos_x: dict.get("startPortalPos.x").unwrap().as_signed_integer().unwrap(),
            start_portal_pos_y: dict.get("startPortalPos.y").unwrap().as_signed_integer().unwrap(),
            highest_point_x: dict.get("highestPoint.x").unwrap().as_signed_integer().unwrap(),
            highest_point_y: dict.get("highestPoint.y").unwrap().as_signed_integer().unwrap(),
            credit: dict.get("credit").unwrap().as_real().unwrap(),
            random_seed: dict.get("randomSeed").unwrap().as_signed_integer().unwrap(),
            no_rain_timer: dict.get("noRainTimer").unwrap().as_real().unwrap(),
            portal_level: dict.get("portalLevel").unwrap().as_signed_integer().unwrap(),
            save_id: dict.get("saveID").unwrap().as_string().unwrap().to_owned(),
            expert_mode: dict.get("expertMode").unwrap().as_boolean().unwrap(),
            minor_version: dict.get("minorVersion").unwrap().as_signed_integer().unwrap(),
            world_width_macro: dict.get("worldWidthMacro").unwrap().as_signed_integer().unwrap()
        }
    }
}