use std::{sync::mpsc::channel, thread::park};

use crate::{
    channels::{Direction, Limit},
    gpio::create_gpio,
    loop_task::loop_task,
};

mod channels;
mod gpio;
mod loop_task;

fn main() {
    println!("Uterus");

    let (direction_tx, direction_rx) = channel::<Direction>();
    let (limit_tx, limit_rx) = channel::<Limit>();

    loop_task(direction_tx, limit_rx);
    create_gpio(direction_rx, limit_tx);

    park();
}
