use crate::dns::proto::qclass::QClass;
use crate::dns::proto::qname::QName;
use crate::dns::proto::qtype::QType;
use crate::dns::proto::reader::Reader;
use crate::dns::proto::writer::Writer;

#[derive(Debug)]
pub struct Question {
    pub qname: QName,
    pub qtype: QType,
    pub class: QClass,
}

impl Question {
    pub fn new(qname: QName, qtype: QType, class: Option<QClass>) -> Question {
        Question {
            qname,
            qtype,
            class: match class {
                Some(class) => class,
                None => QClass::INTERNET,
            },
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        for label in self.qname.labels() {
            let bytes = label.as_bytes().to_vec();
            writer.write_u8(bytes.len() as u8);
            writer.write_vec(&bytes);
        }

        writer.write_u8(0);

        writer.write_u16(self.qtype.to_num());
        writer.write_u16(self.class.to_num());
    }

    pub fn read(reader: &mut Reader) -> Question {
        Question {
            qname: QName::read(reader),
            qtype: QType::from_num(reader.read_u16()),
            class: QClass::from_num(reader.read_u16()),
        }
    }
}
