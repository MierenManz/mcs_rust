pub struct LoginStartPacket {
    pub name: [char; 16],
}

pub struct EncryptionResponsePacket {
    pub shared_secret_length: i32,
    pub shared_secret: Vec<u8>,
    pub verify_token_length: i32,
    pub verify_token: Vec<u8>,
}

pub struct LoginPluginResponsePacket {
    pub message_id: i32,
    pub successful: bool,
    pub data: Option<Vec<u8>>,
}
