
pub struct Ping{
   pub  version: u128,
    pub from:PingData,
    pub to: PingData,
    pub packet_data: PacketData
}

pub struct PingData{
    pub ip_address: String,
    pub upd_port: u128,
    pub tcp_port:u128
}

#[derive(Debug)]
pub struct PacketData{
    pub version: u128,
    pub sender_information: String,
    pub receiver_information: String,
    pub expiration_date: u128,
    pub enr_sequence_number: u128
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

    pub fn new(ip_address: String, upd_port: u128,tcp_port: u128 )->Self{

        Self{
            ip_address,
            upd_port,
            tcp_port
        }
    }
    
}
impl PacketData {

    pub fn new(version: u128, sender_information: String, receiver_information: String,expiration_date: u128, enr_sequence_number: u128)->Self{

        Self{
            version,
            sender_information,
            receiver_information,
            expiration_date,
            enr_sequence_number
        }
    }
    
}