use deku::prelude::*;
use crate::mdr2::Mdr2;
use crate::mdr::Mdr;

/// Basically all
#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum DataType {
    #[deku(id = "0")] Data = 0, // Unused
    #[deku(id = "2")] DataMcNo1 = 2, // Unused
    #[deku(id = "9")] DataIcd = 9, // Unused
    #[deku(id = "10")] DataEv = 10, // Unused
    #[deku(id = "12")] DataMdr = 12,
    #[deku(id = "13")] DataCommon = 13, // Unused
    #[deku(id = "14")] DataMdrNo2 = 14,
    #[deku(id = "1")] Ack = 1,
    #[deku(id = "16")] Shot = 16, // Unused
    #[deku(id = "18")] ShotMcNo1 = 18, // Unused
    #[deku(id = "25")] ShotIcd = 25, // Unused
    #[deku(id = "26")] ShotEv = 26, // Unused?
    #[deku(id = "28")] ShotMdr = 28,
    #[deku(id = "29")] ShotCommon = 29, // Unused
    #[deku(id = "30")] ShotMdrNo2 = 30,
    #[deku(id = "45")] LargeDataCommon = 45, // Unused
}

impl DataType {
    fn needs_ack(&self) -> bool {
        match self {
            DataType::Data => true,
            DataType::DataMcNo1 => true,
            DataType::DataIcd => true,
            DataType::DataEv => true,
            DataType::DataMdr => true,
            DataType::DataCommon => true,
            DataType::DataMdrNo2 => true,
            DataType::Ack => false,
            DataType::Shot => false,
            DataType::ShotMcNo1 => false,
            DataType::ShotIcd => false,
            DataType::ShotEv => false,
            DataType::ShotMdr => false,
            DataType::ShotCommon => false,
            DataType::ShotMdrNo2 => false,
            DataType::LargeDataCommon => true,
        }
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Packet {
    pub data_type: DataType,
    pub seq_no: u8,

    #[deku(endian = "big", update = "self.data.len()")]
    pub data_len: u32,

    #[deku(count = "data_len")]
    pub data: Vec<u8>,

    #[deku(update = "self.calc_checksum()")]
    pub checksum: u8,
}

static PACKET_ACK_0: Packet = Packet {
    data_type: DataType::Ack,
    seq_no: 0,
    data_len: 0,
    data: Vec::new(),
    checksum: 1,
};

static PACKET_ACK_1: Packet = Packet {
    data_type: DataType::Ack,
    seq_no: 1,
    data_len: 0,
    data: Vec::new(),
    checksum: 2,
};

impl Packet {
    pub fn new_cmd(cmd: &Mdr) -> Packet {
        let cmd_data = cmd.to_bytes().unwrap();

        let mut p = Packet {
            data_type: DataType::DataMdr,
            seq_no: 0,
            data_len: cmd_data.len() as u32,
            data: cmd_data,
            checksum: 0,
        };
        p.correct_checksum();

        p
    }

    pub fn read_mdr(&self) -> Mdr {
        assert!(self.data_type == DataType::DataMdr || self.data_type == DataType::ShotMdr);

        let ((remaining, _), c): ((&[u8], _), Mdr) = Mdr::from_bytes((&self.data, 0)).unwrap();

        if !remaining.is_empty() {
            println!("WARNING: Unexpected leftover Command bytes. {:?}", remaining);
        }

        c
    }

    pub fn read_mdr2(&self) -> Mdr2 {
        assert!(self.data_type == DataType::DataMdrNo2 || self.data_type == DataType::ShotMdrNo2);

        let ((remaining, _), c): ((&[u8], _), Mdr2) = Mdr2::from_bytes((&self.data, 0)).unwrap();

        if !remaining.is_empty() {
            println!("WARNING: Unexpected leftover Command bytes. {:?}", remaining);
        }

        c
    }

    pub fn calc_checksum(&self) -> u8 {
        (self.data_type as u8)
            .wrapping_add(self.seq_no)
            .wrapping_add((self.data.len() as u32)
                .to_be_bytes()
                .iter()
                .fold(0, |acc, &x| acc.wrapping_add(x)))
            .wrapping_add(self.data
                .iter()
                .fold(0, |acc, &x| acc.wrapping_add(x)))
    }

    pub fn correct_checksum(&mut self) {
        self.checksum = self.calc_checksum();
    }

    pub fn needs_ack(&self) -> bool {
        self.data_type.needs_ack()
    }

    pub fn get_ack_for(&self) -> &'static Packet {
        if self.seq_no == 0 {
            &PACKET_ACK_1
        } else {
            &PACKET_ACK_0
        }
    }
}
