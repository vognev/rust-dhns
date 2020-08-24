use crate::support::HeadersBag;
use std::io::prelude::*;
use std::io::BufReader;

pub struct ContainersList<T> {
    io: T,
}

impl<T> ContainersList<T>
where
    T: Read + Write,
{
    pub fn new(io: T) -> ContainersList<T> {
        ContainersList { io }
    }

    pub fn exec(&mut self) -> std::io::Result<String> {
        self.io
            .write_all(b"GET /v1.24/containers/json HTTP/1.0\r\n")?;
        self.io.write_all(b"Host: localhost\r\n")?;
        self.io.write_all(b"\r\n")?;

        let mut reader = BufReader::new(&mut self.io);

        let mut status = String::new();
        reader.read_line(&mut status)?;

        let mut headers = HeadersBag::new();

        loop {
            let mut header = String::new();
            reader.read_line(&mut header)?;
            header = header.trim().to_string();

            if header.is_empty() {
                break;
            } else {
                headers.add_from_string(header);
            }
        }

        if let Some(content_length) = headers.get_first(String::from("content-length")) {
            let body_length: usize = content_length.parse().unwrap();
            let mut body = vec![0u8; body_length];

            reader.read_exact(&mut body)?;
            std::io::Result::Ok(String::from_utf8(body).unwrap())
        } else {
            let mut data = vec![];
            reader.read_to_end(&mut data).unwrap();

            std::io::Result::Ok(String::from_utf8(data).unwrap())
        }
    }
}
