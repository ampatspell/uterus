use std::{cell::OnceCell, collections::HashMap, sync::Arc, thread::spawn};

use crossbeam_channel::{Receiver, Sender};
use feather::{App, middleware_fn, next};

#[middleware_fn]
fn hello() {
    let mut data = HashMap::new();
    data.insert("ok", true);
    res.send_json(&data);
    next!()
}

pub enum ApiMessage {}

pub fn create_api(sender: &Sender<ApiMessage>, receiver: &Receiver<ApiMessage>) {
    let sender = sender.clone();
    let _ = spawn(move || {
        let mut app = App::new();
        app.get("/", hello);
        app.listen("0.0.0.0:3000");
    });
}
