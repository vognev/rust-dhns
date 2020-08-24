use std::collections::HashMap;

#[derive(Debug)]
pub struct HeadersBag {
    headers: HashMap<String, Vec<String>>,
}

impl HeadersBag {
    pub fn new() -> HeadersBag {
        HeadersBag {
            headers: HashMap::new(),
        }
    }

    pub fn add_from_string(&mut self, header: String) {
        if let Some(pos) = header.find(':') {
            let name = &header[..pos].to_lowercase();
            let value = header[pos + 1..].trim().to_string();
            match self.headers.get_mut(name) {
                Some(headers) => headers.push(value),
                None => {
                    self.headers.insert(String::from(name), vec![value]);
                }
            }
        } else {
            // todo
        }
    }

    pub fn get_first(&self, header: String) -> Option<String> {
        if let Some(headers) = self.headers.get(&header) {
            Some(String::from(&headers[0]))
        } else {
            None
        }
    }
}
