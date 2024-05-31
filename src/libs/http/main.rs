use super::connection::HttpConnectionStatus;
use crate::libs::logs::main::LogsInstance;

use std::{
    net::{IpAddr, TcpListener, TcpStream},
    sync::mpsc,
};

pub struct HttpInstance {
    port: u16,
    address: IpAddr,
    status: HttpConnectionStatus,
    listener: Option<TcpListener>,
    thread_channel: Option<mpsc::Sender<HttpInstance>>,
}

impl HttpInstance {
    /// Instanciate a new http connection
    pub fn new(_port: u16, _address: IpAddr) -> HttpInstance {
        let _listener: Option<TcpListener>;
        let _status: HttpConnectionStatus;

        // Try listen to address
        match TcpListener::bind(format!("{}:{}", _address, _port)) {
            // Handling success
            Ok(listener_result) => {
                LogsInstance::print(
                    format!("Http instance will listen to {}:{}", _address, _port).as_str(),
                    colored::Color::White,
                );

                // Instanciate to the structure
                _listener = Some(listener_result);
                _status = HttpConnectionStatus::Success;
            }
            // Handling errors
            Err(err) => {
                LogsInstance::print(
                    format!("Cannot listen to: {}:{} \nReason: {}", _address, _port, err).as_str(),
                    colored::Color::Red,
                );
                _status = HttpConnectionStatus::Failed;
                _listener = None;
            }
        }

        Self {
            port: _port,
            address: _address,
            status: _status,
            listener: _listener,
            thread_channel: None,
        }
    }

    /// Start listening to the address and port set
    pub fn infinity_listen(mut self, channel: mpsc::Sender<HttpInstance>) {
        // Update thread communication channel
        self.thread_channel = Some(channel);

        // Check if listener exist
        match self.listener {
            Some(listener) => {
                LogsInstance::print("Http instance started listening", colored::Color::Green);
                for stream in listener.incoming() {
                    let _stream: TcpStream = stream.unwrap();

                    println!("Connection established!");
                }
            }
            None => {
                LogsInstance::print("Error, cannot listen a bind not set, http requisitions will not be received", colored::Color::Red);
            }
        }
    }

    /// Receive the actual connection status of the instance
    pub fn get_status(self) -> HttpConnectionStatus {
        self.status
    }

    /// Receive the address of the instance
    pub fn get_address(self) -> IpAddr {
        self.address
    }

    /// Receives the port of the instance
    pub fn get_port(self) -> u16 {
        self.port
    }
}
