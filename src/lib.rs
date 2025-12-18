pub mod parser;

use crate::parser::*;

use std::{
    io::Write,
    net::{Ipv4Addr, TcpListener, TcpStream},
};

pub fn run(config: Config) {
    let listener = TcpListener::bind(config.to_socket_string()).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let msg = parse_message(&stream);

    let response = create_response(msg);

    let result = stream.write_all(&response);

    match result {
        Ok(_) => {
            println!("Response successfully written to TCP stream");
        }
        Err(e) => {
            println!("Failed to write response to TCP stream: {}", e);
        }
    }
}

fn create_response(msg: Message) -> Vec<u8> {
    let mut response: Vec<u8> = Vec::with_capacity(8);

    append_response_header(&mut response, &msg.header.corr_id);

    let error_code: i16 = if is_api_ver_valid(&msg.header.req_api_ver) { 0 } else { 35 };

    response.extend_from_slice(&error_code.to_be_bytes());

    append_response_body(&mut response, &msg.header.req_api_key);

    update_msg_size(&mut response);

    response
}

fn append_response_header(response: &mut Vec<u8>, corr_id: &i32) { 
    let msg_size: i32 = 0;

    response.extend_from_slice(&msg_size.to_be_bytes());
    response.extend_from_slice(&corr_id.to_be_bytes());
}

fn append_response_body(response: &mut Vec<u8>, api_key: &i16) {
    let mut body: Vec<u8> = Vec::new();

    let min_ver: i16 = 0;
    let max_ver: i16 = 4;
    let tag_buffer: i8 = 0;
    let throttle_time_ms: i32 = 0;

    body.push(0);
    body.extend_from_slice(&api_key.to_be_bytes());
    body.extend_from_slice(&min_ver.to_be_bytes());
    body.extend_from_slice(&max_ver.to_be_bytes());
    body.extend_from_slice(&tag_buffer.to_be_bytes());
    body.extend_from_slice(&throttle_time_ms.to_be_bytes());
    body.extend_from_slice(&tag_buffer.to_be_bytes());

    response.append(&mut body);
}

fn update_msg_size(response: &mut Vec<u8>) {
    let msg_size: i32 = (response.len() - 4) as i32;
    response[0..4].copy_from_slice(&msg_size.to_be_bytes());
}

fn is_api_ver_valid(api_ver: &i16) -> bool {
    if *api_ver <= 4 {
        true
    } else {
        false
    }
}

pub struct Config {
    host_addr: Ipv4Addr,
    port: u16,
}

impl Config {
    pub fn default() -> Self {
        Config {
            host_addr: Ipv4Addr::new(127, 0, 0, 1),
            port: 9092,
        }
    }

    /// Returns a String representation for socket address v4
    ///
    /// # Examples
    /// ```
    /// let config = codecrafters_kafka::Config::default();
    ///
    /// assert_eq!("127.0.0.1:9092", config.to_socket_string());
    /// ```
    pub fn to_socket_string(&self) -> String {
        format!("{}:{}", self.host_addr, self.port)
    }
}

