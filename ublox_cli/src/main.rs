use std::io;
use std::io::Read;
use ublox::PacketRef::NavStatus;
use ublox::*;

fn main() {
    let reader: Box<dyn Read> = Box::new(io::stdin());
    let mut parser = Parser::default();
    let data: Vec<u8> = reader.bytes().map(|a| a.unwrap()).collect();
    let mut it = parser.consume(&data);
    loop {
        match it.next() {
            Some(Ok(packet)) => {
                // if let NavStatus(packet) = packet {
                //     println!("{:?}", packet.fix_stat());
                // }
                println!("{}", serde_json::to_string_pretty(&packet).unwrap());
            }
            Some(Err(_)) => {}
            None => break,
        }
    }
}
