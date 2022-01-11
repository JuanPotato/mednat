pub use crate::commands::connect::*;
pub use crate::commands::common::*;
use deku::prelude::*;

mod common;
mod connect;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SonyCommand {
    #[deku(id = "0")] ConnectGetProtocolInfo(GetProtocolInfo),
    #[deku(id = "1")] ConnectRetProtocolInfo(RetProtocolInfo),
    #[deku(id = "2")] ConnectGetCapabilityInfo(GetCapabilityInfo),
    #[deku(id = "3")] ConnectRetCapabilityInfo(RetCapabilityInfo),
    #[deku(id = "4")] ConnectGetDeviceInfo(GetDeviceInfo),
    #[deku(id = "5")] ConnectRetDeviceInfo(RetDeviceInfo),
    #[deku(id = "6")] ConnectGetSupportFunction(GetSupportFunction),
    #[deku(id = "7")] ConnectRetSupportFunction(RetSupportFunction),

    #[deku(id = "16")] CommonGetBatteryLevel(GetBatteryLevel),
    #[deku(id = "17")] CommonRetBatteryLevel(RetBatteryLevel),
    #[deku(id = "18")] CommonNtfyBatteryLevel(NtfyBatteryLevel),
    #[deku(id = "20")] CommonGetUpscalingEffect(GetUpscalingEffect),
    #[deku(id = "21")] CommonRetUpscalingEffect(RetUpscalingEffect),
    #[deku(id = "23")] CommonNtfyUpscalingEffect(NtfyUpscalingEffect),
    #[deku(id = "24")] CommonGetAudioCodec(GetAudioCodec),
    #[deku(id = "25")] CommonRetAudioCodec(RetAudioCodec),
    #[deku(id = "27")] CommonNtfyAudioCodec(NtfyAudioCodec),
    #[deku(id = "28")] CommonGetBluetoothDeviceInfo(GetBluetoothDeviceInfo),
    #[deku(id = "29")] CommonRetBluetoothDeviceInfo(RetBluetoothDeviceInfo),
    #[deku(id = "34")] CommonSetPowerOff(SetPowerOff),
    #[deku(id = "36")] CommonGetConnectionStatus(GetConnectionStatus),
    #[deku(id = "37")] CommonRetConnectionStatus(RetConnectionStatus),
    #[deku(id = "39")] CommonNtfyConnectionStatus(NtfyConnectionStatus),
    #[deku(id = "40")] CommonGetConciergeData(GetConciergeData),
    #[deku(id = "41")] CommonRetConciergeData(RetConciergeData),
    #[deku(id = "46")] CommonSetLinkControl(SetLinkControl),
    #[deku(id = "47")] CommonNtfyLinkControl(NtfyLinkControl),

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

    #[deku(id = "80")] EqEbbGetCapability,
    #[deku(id = "81")] EqEbbRetCapability,
    #[deku(id = "82")] EqEbbGetStatus,
    #[deku(id = "83")] EqEbbRetStatus,
    #[deku(id = "85")] EqEbbNtfyStatus,
    #[deku(id = "86")] EqEbbGetParam,
    #[deku(id = "87")] EqEbbRetParam,
    #[deku(id = "88")] EqEbbSetParam,
    #[deku(id = "89")] EqEbbNtfyParam,
    #[deku(id = "90")] EqEbbGetExtendedInfo,
    #[deku(id = "91")] EqEbbRetExtendedInfo,

    #[deku(id = "96")] NcAsmGetCapability,
    #[deku(id = "97")] NcAsmRetCapability,
    #[deku(id = "98")] NcAsmGetStatus,
    #[deku(id = "99")] NcAsmRetStatus,
    #[deku(id = "101")] NcAsmNtfyStatus,
    #[deku(id = "102")] NcAsmGetParam,
    #[deku(id = "103")] NcAsmRetParam,
    #[deku(id = "104")] NcAsmSetParam,
    #[deku(id = "105")] NcAsmNtfyParam,

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
