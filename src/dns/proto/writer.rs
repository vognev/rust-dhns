pub struct Writer<'a> {
    buf: &'a mut Vec<u8>,
}

impl<'a> Writer<'a> {
    pub fn new(buf: &mut Vec<u8>) -> Writer {
        Writer { buf }
    }

    pub fn write_u8(&mut self, byte: u8) {
        self.buf.push(byte);
    }

    pub fn write_u16(&mut self, val: u16) {
        let b0 = ((val & 0xff << 0) >> 0) as u8;
        let b1 = ((val & 0xff << 8) >> 8) as u8;

        self.buf.push(b1);
        self.buf.push(b0);
    }

    pub fn write_u32(&mut self, val: u32) {
        let b0 = ((val & 0xff << 0) >> 0) as u8;
        let b1 = ((val & 0xff << 8) >> 8) as u8;
        let b2 = ((val & 0xff << 16) >> 16) as u8;
        let b3 = ((val & 0xff << 24) >> 24) as u8;
        self.write_vec(&vec![b3, b2, b1, b0]);
    }

    pub fn write_vec(&mut self, vec: &Vec<u8>) {
        let mut idx = 0;

        while idx < vec.len() {
            self.buf.push(vec[idx]);
            idx = idx + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_u8() {
        let mut buf: Vec<u8> = vec![];
        let mut writer = Writer { buf: &mut buf };

        writer.write_u8(42);
        assert_eq!(42, buf[0])
    }
}
