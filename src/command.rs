use deku::prelude::*;
use crate::battery::{GetBatteryLevel, RetBatteryLevel, NtfyBatteryLevel};
use crate::deku_str::DekuString;

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

        let ((remaining, _), c): ((&[u8], _), SonyCommand) = SonyCommand::from_bytes((&self.data, 0)).unwrap();

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

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum CommonCapabilityInquiredType {
    #[deku(id = "0")] FixedValue,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetProtocolInfo {
    pub capability_inquired: CommonCapabilityInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct RetProtocolInfo {
    pub capability_inquired: CommonCapabilityInquiredType,
    #[deku(endian = "big")]
    pub protocol_version: u16,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetCapabilityInfo {
    pub capability_inquired: CommonCapabilityInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct RetCapabilityInfo {
    pub capability_inquired: CommonCapabilityInquiredType,
    pub capability_counter: u8,
    #[deku(
    reader = "DekuString::read(rest)",
    writer = "DekuString::write(output, &self.unique_id)"
    )]
    pub unique_id: String,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum DeviceInfoInquiredType {
    #[deku(id = "0")] NoUse,
    #[deku(id = "1")] ModelName,
    #[deku(id = "2")] FwVersion,
    #[deku(id = "3")] SeriesAndColorInfo,
    #[deku(id = "4")] InstructionGuide,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetDeviceInfo {
    pub info_inquired: DeviceInfoInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum RetDeviceInfo {
    #[deku(id = "0")] NoUse,
    #[deku(id = "1")] ModelName(DeviceInfoModelName),
    #[deku(id = "2")] FwVersion(DeviceInfoFwVersion),
    #[deku(id = "3")] SeriesAndColorInfo(DeviceInfoSeriesAndColorInfo),
    #[deku(id = "4")] InstructionGuide(DeviceInfoInstructionGuide),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct DeviceInfoModelName {
    #[deku(
    reader = "DekuString::read(rest)",
    writer = "DekuString::write(output, &self.model_name)"
    )]
    pub model_name: String,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct DeviceInfoFwVersion {
    #[deku(
    reader = "DekuString::read(rest)",
    writer = "DekuString::write(output, &self.fw_version)"
    )]
    pub fw_version: String,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ModelSeries {
    #[deku(id = "0")] NoSeries,
    #[deku(id = "16")] ExtraBass,
    #[deku(id = "32")] Hear,
    #[deku(id = "48")] Premium,
    #[deku(id = "64")] Sports,
    #[deku(id = "80")] Casual,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ModelColor {
    #[deku(id = "0")] Default,
    #[deku(id = "1")] Black,
    #[deku(id = "2")] White,
    #[deku(id = "3")] Silver,
    #[deku(id = "4")] Red,
    #[deku(id = "5")] Blue,
    #[deku(id = "6")] Pink,
    #[deku(id = "7")] Yellow,
    #[deku(id = "8")] Green,
    #[deku(id = "9")] Gray,
    #[deku(id = "10")] Gold,
    #[deku(id = "11")] Cream,
    #[deku(id = "12")] Orange,
    #[deku(id = "13")] Brown,
    #[deku(id = "14")] Violet,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct DeviceInfoSeriesAndColorInfo {
    pub series: ModelSeries,
    pub color: ModelColor,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum GuidanceCategory {
    #[deku(id = "0")]  ChangeEarpiece,
    #[deku(id = "16")] WearEarphone,
    #[deku(id = "32")] PlayButtonOperation,
    #[deku(id = "48")] TouchPadOperation,
    #[deku(id = "64")] MainBodyOperation,
    #[deku(id = "80")] QuickAttention,
    #[deku(id = "96")] AssignableButtonSettings,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct DeviceInfoInstructionGuide {
    guidance_len: u8,
    #[deku(count = "guidance_len")]
    guidance_categories: Vec<GuidanceCategory>,
}


#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SonyCommand {
    #[deku(id = "0")] ConnectGetProtocolInfo(GetProtocolInfo),
    #[deku(id = "1")] ConnectRetProtocolInfo(RetProtocolInfo),
    #[deku(id = "2")] ConnectGetCapabilityInfo(GetCapabilityInfo),
    #[deku(id = "3")] ConnectRetCapabilityInfo(RetCapabilityInfo),
    #[deku(id = "4")] ConnectGetDeviceInfo(GetDeviceInfo),
    #[deku(id = "5")] ConnectRetDeviceInfo(RetDeviceInfo),
    #[deku(id = "6")] ConnectGetSupportFunction,
    #[deku(id = "7")] ConnectRetSupportFunction,

    #[deku(id = "16")] CommonGetBatteryLevel(GetBatteryLevel),
    #[deku(id = "17")] CommonRetBatteryLevel(RetBatteryLevel),
    #[deku(id = "18")] CommonNtfyBatteryLevel(NtfyBatteryLevel),
    #[deku(id = "20")] CommonGetUpscalingEffect,
    #[deku(id = "21")] CommonRetUpscalingEffect,
    #[deku(id = "23")] CommonNtfyUpscalingEffect,
    #[deku(id = "24")] CommonGetAudioCodec,
    #[deku(id = "25")] CommonRetAudioCodec,
    #[deku(id = "27")] CommonNtfyAudioCodec,
    #[deku(id = "28")] CommonGetBluetoothDeviceInfo,
    #[deku(id = "29")] CommonRetBluetoothDeviceInfo,
    #[deku(id = "34")] CommonSetPowerOff,
    #[deku(id = "36")] CommonGetConnectionStatus,
    #[deku(id = "37")] CommonRetConnectionStatus,
    #[deku(id = "39")] CommonNtfyConnectionStatus,
    #[deku(id = "40")] CommonGetConciergeData,
    #[deku(id = "41")] CommonRetConciergeData,
    #[deku(id = "46")] CommonSetLinkControl,
    #[deku(id = "47")] CommonNtfyLinkControl,

    #[deku(id = "52")] UpdtSetStatus,
    #[deku(id = "53")] UpdtNtfyStatus,
    #[deku(id = "54")] UpdtGetParam,
    #[deku(id = "55")] UpdtRetParam,

    #[deku(id = "64")] VptGetCapability,
    #[deku(id = "65")] VptRetCapability,
    #[deku(id = "66")] VptGetStatus,
    #[deku(id = "67")] VptRetStatus,
    #[deku(id = "69")] VptNtfyStatus,
    #[deku(id = "70")] VptGetParam,
    #[deku(id = "71")] VptRetParam,
    #[deku(id = "72")] VptSetParam,
    #[deku(id = "73")] VptNtfyParam,

    #[deku(id = "80")] EqebbGetCapability,
    #[deku(id = "81")] EqebbRetCapability,
    #[deku(id = "82")] EqebbGetStatus,
    #[deku(id = "83")] EqebbRetStatus,
    #[deku(id = "85")] EqebbNtfyStatus,
    #[deku(id = "86")] EqebbGetParam,
    #[deku(id = "87")] EqebbRetParam,
    #[deku(id = "88")] EqebbSetParam,
    #[deku(id = "89")] EqebbNtfyParam,
    #[deku(id = "90")] EqebbGetExtendedInfo,
    #[deku(id = "91")] EqebbRetExtendedInfo,

    #[deku(id = "96")] NcasmGetCapability,
    #[deku(id = "97")] NcasmRetCapability,
    #[deku(id = "98")] NcasmGetStatus,
    #[deku(id = "99")] NcasmRetStatus,
    #[deku(id = "101")] NcasmNtfyStatus,
    #[deku(id = "102")] NcasmGetParam,
    #[deku(id = "103")] NcasmRetParam,
    #[deku(id = "104")] NcasmSetParam,
    #[deku(id = "105")] NcasmNtfyParam,

    #[deku(id = "112")] SenseGetCapability,
    #[deku(id = "113")] SenseRetCapability,
    #[deku(id = "116")] SenseSetStatus,

    #[deku(id = "128")] OptGetCapability,
    #[deku(id = "129")] OptRetCapability,
    #[deku(id = "130")] OptGetStatus,
    #[deku(id = "131")] OptRetStatus,
    #[deku(id = "132")] OptSetStatus,
    #[deku(id = "133")] OptNtfyStatus,
    #[deku(id = "134")] OptGetParam,
    #[deku(id = "135")] OptRetParam,
    #[deku(id = "137")] OptNtfyParam,

    #[deku(id = "144")] AlertGetCapability,
    #[deku(id = "145")] AlertRetCapability,
    #[deku(id = "148")] AlertSetStatus,
    #[deku(id = "152")] AlertSetParam,
    #[deku(id = "153")] AlertNtfyParam,

    #[deku(id = "160")] PlayGetCapability,
    #[deku(id = "161")] PlayRetCapability,
    #[deku(id = "162")] PlayGetStatus,
    #[deku(id = "163")] PlayRetStatus,
    #[deku(id = "164")] PlaySetStatus,
    #[deku(id = "165")] PlayNtfyStatus,
    #[deku(id = "166")] PlayGetParam,
    #[deku(id = "167")] PlayRetParam,
    #[deku(id = "168")] PlaySetParam,
    #[deku(id = "169")] PlayNtfyParam,

    #[deku(id = "176")] SportsGetCapability,
    #[deku(id = "177")] SportsRetCapability,
    #[deku(id = "178")] SportsGetStatus,
    #[deku(id = "179")] SportsRetStatus,
    #[deku(id = "181")] SportsNtfyStatus,
    #[deku(id = "182")] SportsGetParam,
    #[deku(id = "183")] SportsRetParam,
    #[deku(id = "184")] SportsSetParam,
    #[deku(id = "185")] SportsNtfyParam,
    #[deku(id = "186")] SportsGetExtendedParam,
    #[deku(id = "187")] SportsRetExtendedParam,
    #[deku(id = "188")] SportsSetExtendedParam,
    #[deku(id = "189")] SportsNtfyExtendedParam,

    #[deku(id = "196")] LogSetStatus,
    #[deku(id = "201")] LogNtfyParam,

    #[deku(id = "208")] GeneralSettingGetCapability,
    #[deku(id = "209")] GeneralSettingRetCapability,
    #[deku(id = "210")] GeneralSettingGetStatus,
    #[deku(id = "211")] GeneralSettingRetStatus,
    #[deku(id = "213")] GeneralSettingNtfyStatus,
    #[deku(id = "214")] GeneralSettingGetParam,
    #[deku(id = "215")] GeneralSettingRetParam,
    #[deku(id = "216")] GeneralSettingSetParam,
    #[deku(id = "217")] GeneralSettingNtnyParam,

    #[deku(id = "224")] AudioGetCapability,
    #[deku(id = "225")] AudioRetCapability,
    #[deku(id = "226")] AudioGetStatus,
    #[deku(id = "227")] AudioRetStatus,
    #[deku(id = "229")] AudioNtfyStatus,
    #[deku(id = "230")] AudioGetParam,
    #[deku(id = "231")] AudioRetParam,
    #[deku(id = "232")] AudioSetParam,
    #[deku(id = "233")] AudioNtfyParam,

    #[deku(id = "240")] SystemGetCapability,
    #[deku(id = "241")] SystemRetCapability,
    #[deku(id = "242")] SystemGetStatus,
    #[deku(id = "243")] SystemRetStatus,
    #[deku(id = "245")] SystemNtfyStatus,
    #[deku(id = "246")] SystemGetParam,
    #[deku(id = "247")] SystemRetParam,
    #[deku(id = "248")] SystemSetParam,
    #[deku(id = "249")] SystemNtfyParam,
    #[deku(id = "250")] SystemGetExtendedParam,
    #[deku(id = "251")] SystemRetExtendedParam,
    #[deku(id = "252")] SystemSetExtendedParam,
    #[deku(id = "253")] SystemNtfyExtendedParam,

    #[deku(id = "255")] TestCommand,
}

// Battery Things

