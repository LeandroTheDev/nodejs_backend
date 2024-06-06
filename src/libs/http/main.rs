#![allow(dead_code)]
use super::connection::{HttpConnectionStatus, HttpHandler};
use crate::libs::logs::main::LogsInstance;

use std::{
    io::{Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    sync::mpsc,
};

pub struct HttpInstance {
    port: u16,
    address: IpAddr,
    status: HttpConnectionStatus,
    listener: Option<TcpListener>,
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
        }
    }

    /// Start listening to the address and port set calling the function handle_http_stream
    pub fn infinity_listen(self, channel: mpsc::Sender<HttpInstance>) {
        // Check if listener exist
        match self.listener {
            Some(listener) => {
                LogsInstance::print(
                    format!(
                        "Http instance started listening in {}:{}",
                        self.address, self.port
                    )
                    .as_str(),
                    colored::Color::Green,
                );

                // Try to listen the stream
                for stream in listener.incoming() {
                    match stream {
                        // Handling the http stream
                        Ok(stream) => handle_http_stream(stream, channel.clone()),
                        Err(_) => return,
                    };
                }
            }
            None => {
                LogsInstance::print(
                    "Error, cannot listen a bind not set, http requisitions will not be received",
                    colored::Color::Red,
                );
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

    /// Returns the response if exist
    pub fn get_response() -> HttpHandler {
        HttpHandler::Account
    }
}

fn handle_http_stream(mut stream: TcpStream, channel: mpsc::Sender<HttpInstance>) {
    // 512 bytes limit for the buffer
    let mut buffer: [u8; 512] = [0; 512];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            // Read received response as utf8
            let request = match std::str::from_utf8(&buffer[..bytes_read]) {
                Ok(character) => character,
                Err(_) => "NULL",
            };

            println!("Requisição recebida: {}", request);
        }
        Err(_) => {
            let error_response = "HTTP/1.1 400 Bad Request\r\n\r\nOverflow";
            _ = stream.write(error_response.as_bytes());
            _ = stream.flush();
        }
    }
}
