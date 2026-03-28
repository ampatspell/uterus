use crate::buttons::Button;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

pub enum Random {
    Button { button: Button, on: bool },
}

pub static RANDOM: Channel<CriticalSectionRawMutex, Random, 5> = Channel::new();
