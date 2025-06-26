mod types;
use std::{net::IpAddr, str::FromStr};
use crate::types::types::{Ping, PacketData, PingData, Pong};
fn main() {
    let sender_ip =IpAddr::from_str("192.168.0.1").unwrap();
    let receiver_ip =IpAddr::from_str("192.168.0.2").unwrap();
    let sender = PingData::new(sender_ip, 30303, 30304);
    let receiver = PingData::new(receiver_ip, 30303, 0);

    let version = 4;
    
    let packet_data = PacketData::new(version, sender.ip_address.clone(), receiver.ip_address,  1720965600, 247);

    // Pong reply
    let pong = Pong::new(sender.clone(), String::from("6fa5d3b0e4f0fce9b92cf388e37fa216e4c47e70f8c6cfc4ffb979a6dc430a6a"),
                                   1720965600, 42);
    println!("{:?}", packet_data);
}
