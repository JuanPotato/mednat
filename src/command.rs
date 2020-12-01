use deku::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum DataType {
    #[deku(id = "0")] Data = 0,
    #[deku(id = "2")] DataMcNo1 = 2,
    #[deku(id = "9")] DataIcd = 9,
    #[deku(id = "10")] DataEv = 10,
    #[deku(id = "12")] DataMdr = 12,
    #[deku(id = "13")] DataCommon = 13,
    #[deku(id = "14")] DataMdrNo2 = 14,
    #[deku(id = "1")] Ack = 1,
    #[deku(id = "16")] Shot = 16,
    #[deku(id = "18")] ShotMcNo1 = 18,
    #[deku(id = "25")] ShotIcd = 25,
    #[deku(id = "26")] ShotEv = 26,
    #[deku(id = "28")] ShotMdr = 28,
    #[deku(id = "29")] ShotCommon = 29,
    #[deku(id = "30")] ShotMdrNo2 = 30,
    #[deku(id = "45")] LargeDataCommon = 45,
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
    pub fn new_cmd(cmd: &SonyCommand) -> Packet {
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

    pub fn read_cmd(&self) -> SonyCommand {
        if self.data_type != DataType::DataMdr {
            panic!("Yikes");
        }

        SonyCommand::from_bytes((&self.data, 0)).unwrap().1
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

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SonyCommand {
    #[deku(id = "16")] GetBatteryLevel(GetBatteryLevel),
    #[deku(id = "17")] RetBatteryLevel(RetBatteryLevel),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum BatteryType {
    #[deku(id = "0")] Battery = 0,
    #[deku(id = "1")] LeftRight = 1,
    #[deku(id = "2")] Cradle = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum BatteryChargingStatus {
    #[deku(id = "0")] NotCharging = 0,
    #[deku(id = "1")] Charging = 1,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetBatteryLevel {
    pub battery_type: BatteryType,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum RetBatteryLevel {
    #[deku(id = "0")] Battery(BatteryLevel),
    #[deku(id = "1")] LeftRight(LeftRightBatteryLevel),
    #[deku(id = "2")] Cradle(CradleBatteryLevel),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct BatteryLevel {
    pub battery_level: u8,
    pub charging_status: BatteryChargingStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct LeftRightBatteryLevel {
    pub left_battery_level: u8,
    pub left_charging_status: BatteryChargingStatus,
    pub right_battery_level: u8,
    pub right_charging_status: BatteryChargingStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct CradleBatteryLevel {
    pub battery_level: u8,
    pub charging_status: BatteryChargingStatus,
}

