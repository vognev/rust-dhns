use std::net::Ipv4Addr;
use std::net::UdpSocket;

use crate::dns::proto::message::Message;
use crate::dns::proto::qname::QName;
use crate::dns::proto::qtype::QType;
use crate::dns::proto::question::Question;

#[derive(Debug)]
pub enum Protocol {
    UDP,
    TCP,
}

#[derive(Debug)]
pub struct Nameserver {
    addr: Ipv4Addr,
    proto: Protocol,
}

impl Nameserver {
    pub fn new(addr: &str, proto: Protocol) -> Nameserver {
        let ip_addr = addr.parse().unwrap();
        Nameserver {
            addr: ip_addr,
            proto,
        }
    }

    pub fn resolve(&self, qname: QName, qtype: QType) -> Result<Message, &str> {
        let mut msg = Message::new();
        msg.ask(Question::new(qname, qtype, None));

        let mut buf: Vec<u8> = vec![];
        msg.write(&mut buf);

        let sock = UdpSocket::bind("0.0.0.0:0").unwrap();

        let mut dest = self.addr.to_string();
        dest.push_str(":53");

        sock.send_to(&buf, dest).unwrap();

        let mut data = [0u8; 512];
        let len = sock.recv(&mut data).unwrap();

        Message::read(&data[..len].to_vec())
    }
}
