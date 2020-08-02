use crate::dns::proto::reader::Reader;

pub struct QName {
    labels: Vec<String>,
}

impl QName {
    pub fn new(labels: Vec<String>) -> QName {
        QName { labels }
    }

    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }

    pub fn fqdn(&self) -> String {
        self.labels.join(".")
    }

    pub fn read(reader: &mut Reader) -> QName {
        let mut labels: Vec<String> = vec![];
        let mut retpos = 0usize;

        loop {
            let len = reader.read_u8() as usize;

            if len == 0 {
                break;
            }

            if 0b11 == (len >> 6) {
                let offset = ((len - 192) << 8) | (reader.read_u8() as usize);
                let oldpos = reader.seek(offset);
                if 0 == retpos {
                    // store position of buffer on first pointer seek
                    retpos = oldpos;
                }
                continue;
            } else {
                labels.push(reader.read_str(len));
            }
        }

        if retpos > 0 {
            // restore buffer position after pointer seek
            reader.seek(retpos);
        }

        QName { labels }
    }

    pub fn from_str(string: &str) -> QName {
        QName {
            labels: String::from(string)
                .split(".")
                .map(|s| String::from(s))
                .collect(),
        }
    }
}

impl std::fmt::Debug for QName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fqdn())
    }
}
