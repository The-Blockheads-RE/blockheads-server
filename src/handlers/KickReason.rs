pub struct KickReason {
    pub display_message: String
}

impl KickReason {
    pub fn from_code(code: u8) -> Self {
        return Self {
            display_message: match code {
                1 => "That username is in use by another device.\nYou can use the same username on multiple devices if you are logged in to the same iCloud or Game Center account.".to_owned(),
                2 => "Another player is currently logged in with the same username.".to_owned(),
                3 => "Please update to the latest version of The Blockheads.\n\nThis version of The Blockheads is older than the version you are trying to connect to.".to_owned(),
                4 => "Please ask the host to update to the latest version of The Blockheads.\n\nThis version is newer than the version you are trying to connect to.".to_owned(),
                5 => "The server has reached the maximum number of players. Please try again later.".to_owned(),
                6 => "The server has banned you from connecting.".to_owned(),
                7 => "The server needs to add you to their whitelist before you can connect.".to_owned(),
                8 => "The server is currently running actions for another username for this device. Only one username can be active from a single device at one time.".to_owned(),
                9 => "The server rejected your connection request as it contained an incorrect key. Please try again later.".to_owned(),
                10 => "Incorrect password.".to_owned(),
                11 => "You have been kicked.".to_owned(),
                12 => "You have been banned.".to_owned(),
                14 => "Your username is invalid. Please use a different name.".to_owned(),
                15 => "The server is currently starting up.\n\nPlease try again.".to_owned(),
                _ => "The server rejected your connection request. Please try again later.".to_owned()
            }
        }
    }
}