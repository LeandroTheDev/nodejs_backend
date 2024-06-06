mod account;
mod libs;
use libs::{http::main::HttpInstance, logs};
use logs::main::LogsInstance;
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::mpsc,
    thread,
};

fn main() {
    // Initializing http instance
    let http_instance: HttpInstance =
        HttpInstance::new(7979, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    // Http listener instance
    thread::spawn(move || {
        // Instanciating communication channel between structs
        let (sender, receiver): (mpsc::Sender<HttpInstance>, mpsc::Receiver<HttpInstance>) = mpsc::channel();
        // Start listening http responses
        thread::spawn(move || http_instance.infinity_listen(sender));
    });

    LogsInstance::print("Starting main thread", colored::Color::White);
    // Main Thread
    loop {}
}
