use dhns::dns::client::{Nameserver, Protocol};
use dhns::dns::proto::qname::QName;
use dhns::dns::proto::qtype::QType;
use std::env;

fn usage() {
    let mut exe = String::from("client");
    match env::current_exe() {
        Ok(ref path) => match path.file_name() {
            Some(filename) => match filename.to_str() {
                Some(name) => exe = String::from(name),
                None => (),
            },
            None => {}
        },
        Err(_) => (),
    };

    print!("Usage: {} [qtype] [qname] <addr>\n", exe);
    std::process::exit(-1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return usage();
    }

    let qtype = QType::from_str(&args[1].to_ascii_uppercase()).unwrap();
    let qname = &args[2];
    let addr = match args.get(3) {
        Some(arg) => String::from(arg),
        None => String::from("127.0.0.1"),
    };

    let ns = Nameserver::new(&addr, Protocol::UDP);

    let msg = ns.resolve(QName::from_str(qname), qtype);

    print!("{:#?}", msg);
}
