use crate::datatypes::varint_enums::NextState;

pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_address: [char; 255],
    pub server_port: u16,
    pub next_state: NextState,
}

pub struct LegacyServerListPingPacket {
    pub payload: u8,
}
