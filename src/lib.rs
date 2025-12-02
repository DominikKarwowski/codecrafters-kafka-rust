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
    let response = create_response();

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

fn create_response() -> Vec<u8> {
    let mut response: Vec<u8> = Vec::with_capacity(8);

    let msg_size: u32 = 0;
    let corr_id: u32 = 7;

    response.copy_from_slice(&msg_size.to_be_bytes());
    response.copy_from_slice(&corr_id.to_be_bytes());

    response
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
