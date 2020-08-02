use crate::dns::proto::qclass::QClass;
use crate::dns::proto::qname::QName;
use crate::dns::proto::qtype::QType;
use crate::dns::proto::reader::Reader;
use crate::dns::proto::writer::Writer;

use std::net::Ipv4Addr;

#[derive(Debug)]
pub enum Record {
    UNKNOWN {
        qname: QName,
        qtype: QType,
        class: QClass,
        ttl: u32,
        rdata: Vec<u8>,
    },
    A {
        qname: QName,
        class: QClass,
        ttl: u32,
        addr: Ipv4Addr,
    },
    NS {
        qname: QName,
        class: QClass,
        ttl: u32,
        nsdname: QName,
    },
    SOA {
        qname: QName,
        class: QClass,
        ttl: u32,
        mname: QName,
        rname: QName,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },
    CNAME {
        qname: QName,
        class: QClass,
        ttl: u32,
        cname: QName,
    },
    PTR {
        qname: QName,
        class: QClass,
        ttl: u32,
        ptrdname: QName,
    },
    MX {
        qname: QName,
        class: QClass,
        ttl: u32,
        preference: u16,
        exchange: QName,
    },
    TXT {
        qname: QName,
        class: QClass,
        ttl: u32,
        data: String,
    },
    Option {
        payload_size: u16,
        rcode: u32,
        rdata: Vec<u8>,
    },
}

impl Record {
    pub fn write(&self, writer: &mut Writer) {
        match self {
            Record::Option {
                payload_size,
                rcode,
                rdata,
            } => {
                writer.write_u8(0);
                writer.write_u16(QType::OPTION.to_num());
                writer.write_u16(*payload_size);
                writer.write_u32(*rcode);
                writer.write_u16(rdata.len() as u16);
                writer.write_vec(rdata);
            }
            _ => unimplemented!(),
        }
    }

    pub fn read(reader: &mut Reader) -> Record {
        let qname = QName::read(reader);
        let qtype = QType::from_num(reader.read_u16());
        let class = QClass::from_num(reader.read_u16());
        let ttl = reader.read_u32();
        let rdata_len = reader.read_u16() as usize;

        match qtype {
            QType::A => Record::A {
                qname,
                class,
                ttl,
                addr: Ipv4Addr::new(
                    reader.read_u8(),
                    reader.read_u8(),
                    reader.read_u8(),
                    reader.read_u8(),
                ),
            },
            QType::NS => Record::NS {
                qname,
                class,
                ttl,
                nsdname: QName::read(reader),
            },
            QType::SOA => Record::SOA {
                qname,
                class,
                ttl,
                mname: QName::read(reader),
                rname: QName::read(reader),
                serial: reader.read_u32(),
                refresh: reader.read_u32(),
                retry: reader.read_u32(),
                expire: reader.read_u32(),
                minimum: reader.read_u32(),
            },
            QType::CNAME => Record::CNAME {
                qname,
                class,
                ttl,
                cname: QName::read(reader),
            },
            QType::PTR => Record::PTR {
                qname,
                class,
                ttl,
                ptrdname: QName::read(reader),
            },
            QType::MX => Record::MX {
                qname,
                class,
                ttl,
                preference: reader.read_u16(),
                exchange: QName::read(reader),
            },
            QType::TXT => {
                let data_len = reader.read_u8();

                Record::TXT {
                    qname,
                    class,
                    ttl,
                    data: reader.read_str(data_len as usize),
                }
            }
            QType::OPTION => Record::Option {
                payload_size: class.to_num(),
                rcode: ttl,
                rdata: reader.read_vec(rdata_len),
            },
            _ => Record::UNKNOWN {
                qname,
                qtype,
                class,
                ttl,
                rdata: reader.read_vec(rdata_len),
            },
        }
    }
}
