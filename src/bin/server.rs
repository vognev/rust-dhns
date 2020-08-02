use dhns::dns::proto::message::Message;
use dhns::dns::resolver::Resolver;
use std::net::UdpSocket;

const UDPV4_DNS_MAX: usize = 1500;

fn main() {
    let sock = UdpSocket::bind("127.0.0.1:1053").expect("Unable to listen on 127.0.0.1:1053");
    let mut buf = [0; UDPV4_DNS_MAX + 1];

    let resolver = Resolver::new();

    println!("Listening on {}", sock.local_addr().unwrap());

    loop {
        match sock.recv_from(&mut buf) {
            Ok((amt, _)) if amt > UDPV4_DNS_MAX => println!(
                "Message size violation - received more than {} bytes",
                UDPV4_DNS_MAX
            ),
            Ok((amt, src)) => {
                let data = buf[..amt].to_vec();

                match Message::read(&data) {
                    Ok(qry) => {
                        println!("Questions from {}: {:#?}", src, qry.questions());

                        let ans = resolver.resolve(qry);
                        let mut res: Vec<u8> = vec![];
                        ans.write(&mut res);

                        match sock.send_to(&res[..], src) {
                            Ok(sz) => println!("Sent {} bytes in response", sz),
                            Err(err) => println!("Error sending response: {}", err),
                        }
                    }
                    Err(err) => println!("Error reading message: {}", err),
                }
            }
            Err(err) => println!("recv_from error: {}", err),
        }
    }
}
