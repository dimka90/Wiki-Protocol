
use std::{hash::Hash, net::IpAddr};
pub struct Ping{
   pub  version: u128,
    pub from:PingData,
    pub to: PingData,
    pub packet_data: PacketData
}
#[derive(Debug, Clone)]
pub struct PingData{
    pub ip_address: IpAddr,
    pub upd_port: u128,
    pub tcp_port:u128
}

#[derive(Debug)]
pub struct PacketData{
    pub sender_information: IpAddr,
    pub receiver_information: IpAddr,
    pub expiration_date: u128,
    pub enr_sequence_number: u128
}


//  [to, ping-hash, expiration, enr-seq, ...]
pub struct Pong{
    pub to: PingData,
    pub pin_hash: String,
    pub expiration: u128,
    pub enr_seq: u128
}

impl  Ping {
    pub  fn new(version: u128, from: PingData, to:PingData, packet_data: PacketData) -> Self{

        Self {
            version,
            from,
            to,
            packet_data
        }

    }
}

impl PingData {

    pub fn new(ip_address: IpAddr, upd_port: u128,tcp_port: u128 )->Self{

        Self{
            ip_address,
            upd_port,
            tcp_port
        }
    }
    
}
impl PacketData {

    pub fn new(version: u128, sender_information: IpAddr, receiver_information: IpAddr,expiration_date: u128, enr_sequence_number: u128)->Self{

        Self{
            sender_information,
            receiver_information,
            expiration_date,
            enr_sequence_number
        }
    }
    
}

impl  Pong {

    pub fn new(to: PingData, pin_hash: String, expiration: u128, enr_seq: u128)-> Self{
        Self{
            to,
            pin_hash,
            expiration,
            enr_seq
        }
    }
    
}