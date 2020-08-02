use rand::prelude::*;

use super::reader::Reader;
use super::writer::Writer;

#[derive(Debug)]
pub struct Header {
    /// Packet Identifier
    /// A random identifier is assigned to query packets
    id: u16,
    /// Query Response
    /// true for queries, false for responses
    qr: u8,
    /// Operation Code
    /// Typically always 0, see RFC1035 for details
    opcode: u8,
    /// Authoritative Answer
    /// Set to 1 if the responding server is authoritative - that is,
    /// it "owns" - the domain queried.
    aa: u8,
    /// Truncated Message
    /// Set to 1 if the message length exceeds 512 bytes.
    /// Traditionally a hint that the query can be reissued using TCP,
    /// for which the length limitation doesn't apply.
    tc: u8,
    /// Recursion Desired
    /// Set by the sender of the request if the server should attempt to resolve
    /// the query recursively if it does not have an answer readily available.
    rd: u8,
    /// Recursion Available
    /// Set by the server to indicate whether or not recursive queries are allowed.
    ra: u8,
    /// Reserved
    /// Originally reserved for later use, but now used for DNSSEC queries.
    z: u8,
    /// Response Code
    /// Set by the server to indicate the status of the response, i.e. whether or not
    /// it was successful or failed, and in the latter case providing details
    /// about the cause of the failure.
    rcode: u8,
}

impl Header {
    pub fn new() -> Header {
        Header {
            id: random(),
            qr: 0u8,
            opcode: 0u8,
            aa: 0u8,
            tc: 0u8,
            rd: 0u8,
            ra: 0u8,
            z: 0u8,
            rcode: 0u8,
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.write_u16(self.id);

        writer.write_u8(
            (self.qr << 7) | (self.opcode << 3) | (self.aa << 2) | (self.tc << 1) | (self.rd << 0),
        );
        writer.write_u8((self.ra << 7) | (self.z << 4) | (self.rcode << 0));
    }

    pub fn read(reader: &mut Reader) -> Header {
        let id = reader.read_u16();

        let byte = reader.read_u8();
        let qr = (byte & (0x1 << 7)) >> 7;
        let opcode = (byte & (0xF << 3)) >> 3;
        let aa = (byte & (0x1 << 2)) >> 2;
        let tc = (byte & (0x1 << 1)) >> 1;
        let rd = (byte & (0x1 << 0)) >> 0;

        let byte = reader.read_u8();
        let ra = (byte & (0x1 << 7)) >> 7;
        let z = (byte & (0x7 << 4)) >> 4;
        let rcode = (byte & (0xF << 0)) >> 0;

        Header {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
        }
    }
}
