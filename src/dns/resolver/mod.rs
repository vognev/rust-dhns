use crate::dns::proto::message::Message;

pub struct Resolver {}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {}
    }

    pub fn resolve(&self, msg: Message) -> Message {
        msg
    }
}
