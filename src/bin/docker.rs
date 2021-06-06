use dhns::docker::request::containers_list::ContainersList;
use dhns::support::Parser;
use std::os::unix::net::UnixStream;

fn main() {
    let stream =
        UnixStream::connect("/var/run/docker.sock").expect("Unable to connect to docker socker");

    let mut request = ContainersList::new(&stream);

    let response = request.exec().unwrap();

    let json = Parser::parse(Vec::from(response));

    dbg!(json.unwrap());
}
