use serde::Deserialize;
use serde::Serialize;

pub enum State {
    Play,
    Status,
    Login,
    Connecting,
}

pub trait Packet<'a>: Serialize + Deserialize<'a> {
    const PACKED_ID: i32;
    const STATE: State;
}
