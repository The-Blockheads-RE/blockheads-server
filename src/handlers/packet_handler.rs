pub struct BasePacket {

}

pub trait Codec {
    fn encode(&self) -> Vec<u8>;
    fn decode(&self) -> Option<T>;
}

impl BasePacket {}