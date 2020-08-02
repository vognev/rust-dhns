pub struct Reader<'a> {
    pos: usize,
    buf: &'a Vec<u8>,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a Vec<u8>) -> Reader {
        Reader { pos: 0, buf }
    }

    pub fn seek(&mut self, pos: usize) -> usize {
        let old_pos = self.pos;
        self.pos = pos;
        old_pos
    }

    pub fn read_u32(&mut self) -> u32 {
        let b0 = self.buf[self.pos + 0] as u32;
        let b1 = self.buf[self.pos + 1] as u32;
        let b2 = self.buf[self.pos + 2] as u32;
        let b3 = self.buf[self.pos + 3] as u32;
        self.pos = self.pos + 4;
        (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
    }

    pub fn read_u16(&mut self) -> u16 {
        let b0 = self.buf[self.pos + 0] as u16;
        let b1 = self.buf[self.pos + 1] as u16;
        self.pos = self.pos + 2;
        (b0 << 8) | b1
    }

    pub fn read_u8(&mut self) -> u8 {
        self.pos = self.pos + 1;
        self.buf[self.pos - 1]
    }

    pub fn read_vec(&mut self, len: usize) -> Vec<u8> {
        self.pos = self.pos + len;
        self.buf[self.pos - len..self.pos].to_vec()
    }

    pub fn read_str(&mut self, len: usize) -> String {
        self.pos = self.pos + len;

        String::from_utf8(self.buf[self.pos - len..self.pos].to_vec()).unwrap()
    }

    pub fn reminder(&self) -> usize {
        self.buf.len() - self.pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buf = vec![];
        let rdr = Reader::new(&buf);

        assert_eq!(0, rdr.pos);
        assert_eq!(0, rdr.buf.len())
    }

    #[test]
    fn test_read_u16() {
        let buf = vec![0xDEu8, 0xAD, 0xBE, 0xEF];
        let mut rdr = Reader::new(&buf);

        assert_eq!(0xDEAD, rdr.read_u16());
        assert_eq!(0xBEEF, rdr.read_u16());
    }

    #[test]
    fn test_read_u8() {
        let buf = vec![0xDEu8, 0xAD];
        let mut rdr = Reader::new(&buf);

        assert_eq!(0xDE, rdr.read_u8());
        assert_eq!(0xAD, rdr.read_u8());
    }
}
