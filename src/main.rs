use std::{
    sync::{Arc, mpsc::channel},
    thread::park,
};

use crate::{channels::Message, gpio::create_gpio, loop_task::loop_task};

mod channels;
mod gpio;
mod loop_task;

fn main() {
    let (tx, rx) = channel::<Message>();
    let tx = Arc::new(tx);

    println!("Uterus");
    loop_task(&tx);
    create_gpio(rx);
    park();
}
