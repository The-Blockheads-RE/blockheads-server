#[derive(std::fmt::Debug)]
pub struct WorldHeartbeat {
    pub world_time: f32,
    pub no_rain_timer: f32,
    pub fast_forward: bool,
    pub local_paused: bool,
    pub all_paused: bool,
    pub pvp_disabled: bool,
    pub credit: f32
}

impl WorldHeartbeat {
    pub fn encode(&self) -> Vec<u8> {
        let mut encoded_data = Vec::new();

        let mut world_timer_bytes = self.world_time.to_le_bytes().to_vec();
        let mut no_rain_timer_bytes = self.no_rain_timer.to_le_bytes().to_vec();
        let mut credit_bytes = self.credit.to_le_bytes().to_vec();

        encoded_data.append(&mut world_timer_bytes);
        encoded_data.append(&mut no_rain_timer_bytes);

        encoded_data.insert(8, self.fast_forward as u8);
        encoded_data.insert(9, self.local_paused as u8);
        encoded_data.insert(10, self.all_paused as u8);
        encoded_data.insert(11, self.pvp_disabled as u8);

        encoded_data.append(&mut credit_bytes);

        return encoded_data;
    }
    pub fn decode(raw_data: Vec<u8>) -> Self {
        return Self {
            world_time: f32::from_le_bytes([raw_data[0], raw_data[1], raw_data[2], raw_data[3]]),
            no_rain_timer: f32::from_le_bytes([raw_data[4], raw_data[5], raw_data[6], raw_data[7]]),
            fast_forward: raw_data[8] == 1,
            local_paused: raw_data[9] == 1,
            all_paused: raw_data[10] == 1,
            pvp_disabled: raw_data[11] == 1,
            credit: f32::from_le_bytes([raw_data[12], raw_data[13], raw_data[14], raw_data[15]])
        }
    }
}