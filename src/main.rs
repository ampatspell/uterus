use crossbeam_channel::unbounded;

use crate::{
    api::{ApiMessage, create_api},
    channels::Message,
    gpio::create_gpio,
    loop_task::loop_task,
};
use std::{
    sync::{Arc, mpsc::channel},
    thread::park,
};

mod api;
mod channels;
mod gpio;
mod loop_task;

fn main() {
    let (tx, rx) = channel::<Message>();
    let tx = Arc::new(tx);

    let (sender, receiver) = unbounded::<ApiMessage>();

    create_api(&sender, &receiver);

    println!("Uterus");
    loop_task(&tx);
    create_gpio(rx);
    park();
}
