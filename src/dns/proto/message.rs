use super::header::Header;
use super::question::Question;
use crate::dns::proto::reader::Reader;
use crate::dns::proto::record::Record;
use crate::dns::proto::writer::Writer;

#[derive(Debug)]
pub struct Message {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<Record>,
    authority: Vec<Record>,
    additional: Vec<Record>,
}

impl Message {
    pub fn questions(&self) -> &Vec<Question> {
        &self.questions
    }

    pub fn new() -> Message {
        Message {
            header: Header::new(),
            questions: vec![],
            answers: vec![],
            authority: vec![],
            additional: vec![],
        }
    }

    pub fn ask(&mut self, question: Question) {
        self.questions.push(question);
    }

    pub fn write(&self, buf: &mut Vec<u8>) {
        let mut writer = Writer::new(buf);

        self.header.write(&mut writer);

        writer.write_u16(self.questions.len() as u16);
        writer.write_u16(self.answers.len() as u16);
        writer.write_u16(self.authority.len() as u16);
        writer.write_u16(self.additional.len() as u16);

        for question in self.questions.iter() {
            question.write(&mut writer);
        }

        for answer in self.answers.iter() {
            answer.write(&mut writer)
        }

        for authority in self.authority.iter() {
            authority.write(&mut writer)
        }

        for additional in self.additional.iter() {
            additional.write(&mut writer)
        }
    }

    pub fn read(buffer: &Vec<u8>) -> Result<Message, &'static str> {
        let mut reader = Reader::new(buffer);

        let header = Header::read(&mut reader);

        let mut qdcount = reader.read_u16();
        let mut ancount = reader.read_u16();
        let mut nscount = reader.read_u16();
        let mut arcount = reader.read_u16();

        let mut questions: Vec<Question> = vec![];
        while qdcount != 0 {
            questions.push(Question::read(&mut reader));
            qdcount = qdcount - 1;
        }

        let mut answers: Vec<Record> = vec![];
        while ancount != 0 {
            answers.push(Record::read(&mut reader));
            ancount = ancount - 1;
        }

        let mut authority = vec![];
        while nscount != 0 {
            authority.push(Record::read(&mut reader));
            nscount = nscount - 1;
        }

        let mut additional: Vec<Record> = vec![];
        while arcount != 0 {
            additional.push(Record::read(&mut reader));
            arcount = arcount - 1;
        }

        if reader.reminder() > 0 {
            return Err("Message was not read completely");
        }

        Ok(Message {
            header,
            questions,
            answers,
            authority,
            additional,
        })
    }
}
