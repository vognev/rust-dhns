use hex::FromHex;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum JsVal {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<JsVal>),
    Object(HashMap<String, JsVal>),
}

#[derive(Debug)]
pub enum JsErr {
    InvalidJson(String),
    ParseError(String),
}

impl fmt::Display for JsErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsErr::InvalidJson(s) => write!(f, "JsErr::InvalidJson {}", s),
            JsErr::ParseError(s) => write!(f, "JsErr::ParseError {}", s),
        }
    }
}

pub struct Parser<'a> {
    data: &'a Vec<u8>,
    idx: usize,
}

type JsResult = Result<JsVal, JsErr>;

impl<'a> Parser<'a> {
    fn init(data: &Vec<u8>) -> Parser {
        Parser { data, idx: 0 }
    }

    fn eof(&mut self) -> bool {
        self.idx >= self.data.len()
    }

    fn char(&mut self) -> char {
        self.data[self.idx] as char
    }

    fn has(&mut self, n: usize) -> bool {
        return self.idx + n < self.data.len();
    }

    fn slice(&mut self, n: usize) -> &[u8] {
        self.idx += n;
        &self.data[self.idx - n..self.idx]
    }

    fn skip(&mut self, n: usize) {
        self.idx += n
    }

    fn parse_literal(&mut self) -> JsResult {
        if 't' == self.char() && self.has(3) {
            if self.data[self.idx + 1] as char == 'r'
                && self.data[self.idx + 2] as char == 'u'
                && self.data[self.idx + 3] as char == 'e'
            {
                self.skip(4);
                return Ok(JsVal::Bool(true));
            }
        }

        if 'f' == self.char() && self.has(4) {
            if self.data[self.idx + 1] as char == 'a'
                && self.data[self.idx + 2] as char == 'l'
                && self.data[self.idx + 3] as char == 's'
                && self.data[self.idx + 4] as char == 'e'
            {
                self.skip(5);
                return Ok(JsVal::Bool(false));
            }
        }

        if 'n' == self.char() && self.has(3) {
            if (self.data[self.idx + 1]) as char == 'u'
                && (self.data[self.idx + 2]) as char == 'l'
                && (self.data[self.idx + 3]) as char == 'l'
            {
                self.skip(4);
                return Ok(JsVal::Null);
            }
        }

        return Err(JsErr::InvalidJson(String::from("Invalid literal")));
    }

    fn parse_number(&mut self) -> JsResult {
        let mut number = String::new();

        if self.char() == '-' {
            number.push('-');
            self.skip(1);
        }

        while !self.eof() {
            let c = self.char();

            match c {
                '0'..='9' => {
                    number.push(c);
                    self.skip(1)
                }
                _ => {
                    break;
                }
            }
        }

        if !self.eof() && self.char() == '.' {
            number.push('.');
            self.skip(1);

            while !self.eof() {
                let c = self.char();

                match c {
                    '0'..='9' => {
                        number.push(c);
                        self.skip(1)
                    }
                    _ => break,
                }
            }

            return match number.parse::<f64>() {
                Ok(nr) => Ok(JsVal::Float(nr)),
                Err(_) => Err(JsErr::ParseError(number)),
            };
        }

        return match number.parse::<i64>() {
            Ok(nr) => Ok(JsVal::Int(nr)),
            Err(_) => Err(JsErr::ParseError(number)),
        };
    }

    fn parse_string(&mut self) -> JsResult {
        let mut string = String::new();
        self.skip(1);

        while !self.eof() {
            match self.char() {
                '"' => {
                    self.skip(1);
                    return Ok(JsVal::String(string));
                }

                '\\' if self.has(1) => {
                    self.skip(1);

                    match self.char() {
                        '"' | '/' | '\\' => {
                            string.push(self.char());
                            self.skip(1)
                        }
                        'b' => {
                            string.push(08 as char);
                            self.skip(1)
                        }
                        'f' => {
                            string.push(12 as char);
                            self.skip(1)
                        }
                        'r' => {
                            string.push('\r');
                            self.skip(1)
                        }
                        'n' => {
                            string.push('\n');
                            self.skip(1)
                        }
                        't' => {
                            string.push('\t');
                            self.skip(1)
                        }

                        'u' if self.has(5) => {
                            self.skip(1);

                            let rune = self.slice(4);

                            match <[u8; 2]>::from_hex(rune) {
                                Ok(rune) => {
                                    let codepoint = (rune[0] as u32) << 8 | (rune[1] as u32);
                                    if let Some(ch) = std::char::from_u32(codepoint) {
                                        string.push(ch);
                                    } else {
                                        return Err(JsErr::InvalidJson(String::from(
                                            "Invalid unicode codepoint",
                                        )));
                                    }
                                }
                                Err(e) => {
                                    return Err(JsErr::ParseError(e.to_string()));
                                }
                            }
                        }

                        _ => {
                            return Err(JsErr::InvalidJson(String::from("Invalid escape")));
                        }
                    }
                }

                _ => {
                    string.push(self.char());
                    self.skip(1)
                }
            }
        }

        return Err(JsErr::InvalidJson(String::from("Invalid string")));
    }

    fn parse_any(&mut self) -> JsResult {
        while !self.eof() {
            match self.char() {
                '\t' | '\r' | '\n' | ' ' => {
                    self.skip(1);
                }

                '[' => {
                    self.skip(1);
                    let mut ary: Vec<JsVal> = vec![];

                    while !self.eof() {
                        match self.char() {
                            ']' => {
                                self.skip(1);
                                return Ok(JsVal::Array(ary));
                            }
                            ',' | '\t' | '\r' | '\n' | ' ' => {
                                self.skip(1);
                            }
                            _ => ary.push(self.parse_any()?),
                        }
                    }

                    return Err(JsErr::InvalidJson(String::from("Invalid array")));
                }

                '{' => {
                    self.skip(1);
                    let mut obj: HashMap<String, JsVal> = HashMap::new();
                    let mut label: Option<String> = None;

                    while !self.eof() {
                        match self.char() {
                            '}' => {
                                self.skip(1);
                                return Ok(JsVal::Object(obj));
                            }
                            '\t' | '\r' | '\n' | ' ' | ',' => {
                                self.skip(1);
                            }
                            ':' => match label {
                                Some(_) => {
                                    self.skip(1);
                                }
                                None => {
                                    return Err(JsErr::InvalidJson(String::from("Unexpected semicolon", )));
                                }
                            },
                            _ => {
                                let val = self.parse_any()?;
                                match label {
                                    None => {
                                        match val {
                                            JsVal::String(lbl) => {
                                                label = Some(lbl);
                                            }
                                            _ => {
                                                return Err(JsErr::InvalidJson(String::from("Unexpected label value")))
                                            }
                                        }
                                        
                                    }
                                    Some(ref l) => {
                                        obj.insert(l.clone(), val); label = None;
                                    }
                                }
                            },
                        }
                    }

                    return Err(JsErr::InvalidJson(String::from("Invalid object")));
                }

                '"' => {
                    return self.parse_string()
                },

                '-' | '0'..='9' => {
                    return self.parse_number()
                },

                't' | 'f' | 'n' => {
                    return self.parse_literal()
                },

                _ => {
                    return Err(JsErr::InvalidJson(format!(
                        "Invalid token: {}", self.char()
                    )));
                }
            }
        }

        return Err(JsErr::InvalidJson(String::from("Invalid JSON")));
    }

    pub fn parse(data: Vec<u8>) -> JsResult {
        let mut parser = Parser::init(&data);
        return parser.parse_any();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_null() {
        let json = "null".as_bytes().to_vec();
        let test = Parser::parse(json);
        assert!(matches!(test, Ok(JsVal::Null)))
    }

    #[test]
    fn test_parse_null_incomplete() {
        let json = "nul".as_bytes().to_vec();
        let test = Parser::parse(json);
        assert!(matches!(test, Err(JsErr::InvalidJson(_))))
    }

    #[test]
    fn test_parse_true() {
        let json = "true".as_bytes().to_vec();
        let test = Parser::parse(json);
        assert!(matches!(test, Ok(JsVal::Bool(true))))
    }

    #[test]
    fn test_parse_true_incomplete() {
        let json = "tru".as_bytes().to_vec();
        let test = Parser::parse(json);
        assert!(matches!(test, Err(JsErr::InvalidJson(_))))
    }

    #[test]
    fn test_parse_false() {
        let json = "false".as_bytes().to_vec();
        let test = Parser::parse(json);
        assert!(matches!(test, Ok(JsVal::Bool(false))))
    }

    #[test]
    fn test_parse_false_incomplete() {
        let json = "fals".as_bytes().to_vec();
        let test = Parser::parse(json);
        assert!(matches!(test, Err(JsErr::InvalidJson(_))))
    }

    #[test]
    fn test_parse_positive_i64() {
        let json = "0042".as_bytes().to_vec();
        let test = Parser::parse(json);
        assert!(matches!(test, Ok(JsVal::Int(42))))
    }

    #[test]
    fn test_parse_negative_i64() {
        let json = "-0042".as_bytes().to_vec();
        let test = Parser::parse(json);
        assert!(matches!(test, Ok(JsVal::Int(-42))))
    }

    #[test]
    fn test_parse_positive_f64() {
        let json = "00.42".as_bytes().to_vec();

        if let Ok(JsVal::Float(nr)) = Parser::parse(json) {
            assert_eq!(0.42, nr)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_parse_negative_f64() {
        let json = "-00.42".as_bytes().to_vec();

        if let Ok(JsVal::Float(nr)) = Parser::parse(json) {
            assert_eq!(-0.42, nr)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_parse_string() {
        let json = "\"\\u00342\\t\\u2764\n\"".as_bytes().to_vec();

        match Parser::parse(json) {
            Ok(parsed) => match parsed {
                JsVal::String(str) => assert_eq!("42\t❤\n", str),
                _ => assert!(false, "Unexpected JsVal"),
            },
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }

    #[test]
    fn test_empty_array() {
        let json = "[]".as_bytes().to_vec();

        match Parser::parse(json) {
            Ok(parsed) => match parsed {
                JsVal::Array(ary) => assert_eq!(0, ary.len()),
                _ => assert!(false, "Unexpected JsVal"),
            },
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }

    #[test]
    fn test_misc_array() {
        let data = "[[]]".as_bytes().to_vec();
        if let Err(e) = Parser::parse(data) {
            assert!(false, "{}", e);
        }
    }

    #[test]
    fn test_mixed_array() {
        let json = "[1, \"test\"]".as_bytes().to_vec();

        match Parser::parse(json) {
            Ok(parsed) => match parsed {
                JsVal::Array(ary) => {
                    assert_eq!(2, ary.len());

                    match ary.get(0) {
                        Some(JsVal::Int(n)) => assert_eq!(1, *n),
                        _ => {
                            assert!(false, "failed");
                        }
                    }

                    match ary.get(1) {
                        Some(JsVal::String(s)) => assert_eq!(String::from("test"), *s),
                        _ => {
                            assert!(false, "failed");
                        }
                    }
                }
                _ => assert!(false, "Unexpected JsVal"),
            },
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }

    #[test]
    fn test_empty_object() {
        let json = "{}".as_bytes().to_vec();

        match Parser::parse(json) {
            Ok(parsed) => match parsed {
                JsVal::Object(map) => assert_eq!(0, map.len()),
                _ => assert!(false, "Unexpected JsVal"),
            },
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }

    #[test]
    fn test_aaaa() {
        let json = "[{\"some\": 1}]".as_bytes().to_vec();

        match Parser::parse(json) {
            Err(e) => {
                assert!(false, "{}", e);
            }
            _ => {}
        }
    }

    #[test]
    fn test_bbbb() {
        let json = "[true, false, null]".as_bytes().to_vec();
        let data = Parser::parse(json);

        if let Err(e) = data {
            assert!(false, "{}", e);
        } 
    }
}
