mod types;

use crate::types::types::{Ping, PacketData, PingData};
fn main() {
    let sender = PingData::new(String::from("192.168.0.1"), 30303, 30304);
    let receiver = PingData::new(String::from("192.168.0.2"), 30303, 0);
    let version = 4;
    let packet_data = PacketData::new(version, sender.ip_address, receiver.ip_address,  1720965600, 247);
    println!("{:?}", packet_data);
}
