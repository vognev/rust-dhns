#[derive(Debug)]
pub enum QType {
    UNKNOWN(u16),
    A,
    NS,
    SOA,
    CNAME,
    PTR,
    MX,
    TXT,
    OPTION,
}

impl QType {
    pub fn from_num(num: u16) -> QType {
        match num {
            1 => QType::A,
            2 => QType::NS,
            6 => QType::SOA,
            5 => QType::CNAME,
            12 => QType::PTR,
            15 => QType::MX,
            16 => QType::TXT,
            41 => QType::OPTION,
            _ => QType::UNKNOWN(num),
        }
    }

    pub fn from_str(qtype: &str) -> Option<QType> {
        match qtype {
            "A" => Some(QType::A),
            "NS" => Some(QType::NS),
            "SOA" => Some(QType::SOA),
            "CNAME" => Some(QType::CNAME),
            "PTR" => Some(QType::PTR),
            "MX" => Some(QType::MX),
            "TXT" => Some(QType::TXT),
            _ => None,
        }
    }

    pub fn to_num(&self) -> u16 {
        match self {
            QType::A => 1,
            QType::NS => 2,
            QType::SOA => 6,
            QType::CNAME => 5,
            QType::PTR => 12,
            QType::MX => 15,
            QType::TXT => 16,
            QType::OPTION => 41,
            QType::UNKNOWN(num) => *num,
        }
    }
}
