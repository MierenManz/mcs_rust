pub mod varint_enums;

pub struct Position {
    x: i32,
    y: i16,
    z: i32,
}

impl Position {
    pub fn new(bytes: i64) -> Self {
        Self {
            x: (bytes >> 38) as i32,
            y: (bytes & 0xFFF) as i16,
            z: ((bytes >> 12) & 0x3FFFFFF) as i32,
        }
    }
}

pub struct Slot {
    pub present: bool,
    item_id: Option<i32>,
    item_count: Option<i8>,
    nbt: Option<String>,
}
