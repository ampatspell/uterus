use std::sync::mpsc::channel;

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

    let tx = direction_tx.clone();
    loop_task(tx, limit_rx);
    create_gpio(direction_rx, limit_tx);

    let (tx, rx) = channel::<()>();
    ctrlc::set_handler(move || {
        direction_tx.send(Direction::Stop).unwrap();
        tx.send(()).unwrap();
    })
    .unwrap();

    rx.recv().unwrap();
}
