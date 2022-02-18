use crate::datatypes::varint_enums::*;
use crate::datatypes::Position;
use crate::datatypes::Slot;

pub struct TeleportConfigPacket {
    pub teleport_id: i32,
}

pub struct QueryBlockNBTPacket {
    pub transaction_id: i32,
    pub location: Position,
}

pub struct SetDifficultyPacket {
    pub new_difficulty: Difficulty,
}

pub struct ChatMessagePacket {
    pub message: [char; 256],
}

pub struct ClientStatusPacket {
    pub action_id: ActionIDs,
}

pub struct ClientSettingsPacket {
    pub locale: [char; 16],
    pub view_distance: i8,
    pub chat_mode: ChatModes,
    pub chat_colors: bool,
    pub displayed_skins_parts: u8,
    pub main_hand: MainHand,
    pub enable_test_filtering: bool,
    pub allow_server_listings: bool,
}

pub struct TabCompletePacket {
    pub transaction_id: i32,
    pub text: [char; 32500],
}

pub struct ClickWindowButtonPacket {
    pub window_id: i8,
    pub button_id: i8,
}

pub struct ClickWindowPacket {
    pub window_id: u8,
    pub state_id: i32,
    pub slot: i16,
    pub button: i8,
    pub mode: u8,
    pub length_of_array: i32,
    pub array_of_slots: Vec<Slot>,
    pub clicked_item: Slot,
}

pub struct ClosedWindowPacket {
    window_id: u8,
}

pub struct PluginMessagePacket {
    channel: String,
    data: Vec<u8>,
}

pub struct EditBookPackage {
    hand: MainHand,
    count: i32,
}
