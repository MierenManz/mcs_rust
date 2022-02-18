pub enum MainHand {
    Left,
    Right,
}

pub enum Hand {
    MainHand,
    OffHand,
}

pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

pub enum NextState {
    Status = 1,
    Login = 2,
}

pub enum ActionIDs {
    PerformRespawn = 0,
    RequestStats = 1,
}

pub enum ChatModes {
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}
