#[derive(Debug)]
pub enum QClass {
    UNKNOWN(u16),
    INTERNET,
}

impl QClass {
    pub fn from_num(num: u16) -> QClass {
        match num {
            1 => QClass::INTERNET,
            _ => QClass::UNKNOWN(num),
        }
    }

    pub fn to_num(&self) -> u16 {
        match self {
            QClass::INTERNET => 1,
            QClass::UNKNOWN(x) => *x,
        }
    }
}
