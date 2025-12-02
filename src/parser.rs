use std::{
    io::{BufReader, Read},
    net::TcpStream,
};

pub fn parse_message(stream: &TcpStream) -> Message {
    let mut reader = BufReader::new(stream);

    let size = read_int32(&mut reader);

    let header = read_header(&mut reader);

    Message::new(size, header)
}

fn read_header(reader: &mut BufReader<&TcpStream>) -> Header {
    let req_api_key = read_int16(reader);
    let req_api_ver = read_int16(reader);
    let corr_id = read_int32(reader);

    Header::new(req_api_key, req_api_ver, corr_id)
}

fn read_int16(reader: &mut BufReader<&TcpStream>) -> i16 {
    let mut buf: [u8; 2] = [0; 2];
    reader.read_exact(&mut buf).unwrap();
    i16::from_be_bytes(buf)
}

fn read_int32(reader: &mut BufReader<&TcpStream>) -> i32 {
    let mut buf: [u8; 4] = [0; 4];
    reader.read_exact(&mut buf).unwrap();
    i32::from_be_bytes(buf)
}

pub struct Message {
    pub size: i32,
    pub header: Header,
}

impl Message {
    fn new(size: i32, header: Header) -> Self {
        Message { size, header }
    }
}

pub struct Header {
    pub req_api_key: i16,
    pub req_api_ver: i16,
    pub corr_id: i32,
}

impl Header {
    fn new(req_api_key: i16, req_api_ver: i16, corr_id: i32) -> Header {
        Header {
            req_api_key,
            req_api_ver,
            corr_id,
        }
    }
}
