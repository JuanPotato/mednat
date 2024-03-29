use deku::prelude::*;

pub mod common;
pub mod connect;
pub mod update;
pub mod eq_ebb;
pub mod nc_asm;
pub mod sense;
pub mod opt;
pub mod alert;
pub mod play;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum Mdr {
    #[deku(id = "0")] ConnectGetProtocolInfo(connect::GetProtocolInfo),
    #[deku(id = "1")] ConnectRetProtocolInfo(connect::RetProtocolInfo),
    #[deku(id = "2")] ConnectGetCapabilityInfo(connect::GetCapabilityInfo),
    #[deku(id = "3")] ConnectRetCapabilityInfo(connect::RetCapabilityInfo),
    #[deku(id = "4")] ConnectGetDeviceInfo(connect::GetDeviceInfo),
    #[deku(id = "5")] ConnectRetDeviceInfo(connect::RetDeviceInfo),
    #[deku(id = "6")] ConnectGetSupportFunction(connect::GetSupportFunction),
    #[deku(id = "7")] ConnectRetSupportFunction(connect::RetSupportFunction),

    #[deku(id = "16")] CommonGetBatteryLevel(common::GetBatteryLevel),
    #[deku(id = "17")] CommonRetBatteryLevel(common::RetBatteryLevel),
    #[deku(id = "18")] CommonNtfyBatteryLevel(common::NtfyBatteryLevel),
    #[deku(id = "20")] CommonGetUpscalingEffect(common::GetUpscalingEffect),
    #[deku(id = "21")] CommonRetUpscalingEffect(common::RetUpscalingEffect),
    #[deku(id = "23")] CommonNtfyUpscalingEffect(common::NtfyUpscalingEffect),
    #[deku(id = "24")] CommonGetAudioCodec(common::GetAudioCodec),
    #[deku(id = "25")] CommonRetAudioCodec(common::RetAudioCodec),
    #[deku(id = "27")] CommonNtfyAudioCodec(common::NtfyAudioCodec),
    #[deku(id = "28")] CommonGetBluetoothDeviceInfo(common::GetBluetoothDeviceInfo),
    #[deku(id = "29")] CommonRetBluetoothDeviceInfo(common::RetBluetoothDeviceInfo),
    #[deku(id = "34")] CommonSetPowerOff(common::SetPowerOff),
    #[deku(id = "36")] CommonGetConnectionStatus(common::GetConnectionStatus),
    #[deku(id = "37")] CommonRetConnectionStatus(common::RetConnectionStatus),
    #[deku(id = "39")] CommonNtfyConnectionStatus(common::NtfyConnectionStatus),
    #[deku(id = "40")] CommonGetConciergeData(common::GetConciergeData),
    #[deku(id = "41")] CommonRetConciergeData(common::RetConciergeData),
    #[deku(id = "46")] CommonSetLinkControl(common::SetLinkControl),
    #[deku(id = "47")] CommonNtfyLinkControl(common::NtfyLinkControl),

    #[deku(id = "52")] UpdtSetStatus(update::SetStatus),
    #[deku(id = "53")] UpdtNtfyStatus(update::NtfyStatus),
    #[deku(id = "54")] UpdtGetParam(update::GetParam),
    #[deku(id = "55")] UpdtRetParam(update::RetParam),

    // Unable to test vpt since my WF-1000XM3's dont support it
    #[deku(id = "64")] VptGetCapability,
    #[deku(id = "65")] VptRetCapability,
    #[deku(id = "66")] VptGetStatus,
    #[deku(id = "67")] VptRetStatus,
    #[deku(id = "69")] VptNtfyStatus,
    #[deku(id = "70")] VptGetParam,
    #[deku(id = "71")] VptRetParam,
    #[deku(id = "72")] VptSetParam,
    #[deku(id = "73")] VptNtfyParam,

    #[deku(id = "80")] EqEbbGetCapability(eq_ebb::EqEbbGetCapability),
    #[deku(id = "81")] EqEbbRetCapability(eq_ebb::EqEbbRetCapability),
    #[deku(id = "82")] EqEbbGetStatus(eq_ebb::EqEbbGetStatus),
    #[deku(id = "83")] EqEbbRetStatus(eq_ebb::EqEbbRetStatus),
    #[deku(id = "85")] EqEbbNtfyStatus(eq_ebb::EqEbbNtfyStatus),
    #[deku(id = "86")] EqEbbGetParam(eq_ebb::EqEbbGetParam),
    #[deku(id = "87")] EqEbbRetParam(eq_ebb::EqEbbRetParam),
    #[deku(id = "88")] EqEbbSetParam(eq_ebb::EqEbbSetParam),
    #[deku(id = "89")] EqEbbNtfyParam(eq_ebb::EqEbbNtfyParam),
    #[deku(id = "90")] EqEbbGetExtendedInfo(eq_ebb::EqEbbGetExtendedInfo),
    #[deku(id = "91")] EqEbbRetExtendedInfo(eq_ebb::EqEbbRetExtendedInfo),

    #[deku(id = "96")] NcAsmGetCapability(nc_asm::NcAsmGetCapability),
    #[deku(id = "97")] NcAsmRetCapability(nc_asm::NcAsmRetCapability),
    #[deku(id = "98")] NcAsmGetStatus(nc_asm::NcAsmGetStatus),
    #[deku(id = "99")] NcAsmRetStatus(nc_asm::NcAsmRetStatus),
    #[deku(id = "101")] NcAsmNtfyStatus(nc_asm::NcAsmNtfyStatus),
    #[deku(id = "102")] NcAsmGetParam(nc_asm::NcAsmGetParam),
    #[deku(id = "103")] NcAsmRetParam(nc_asm::NcAsmRetParam),
    #[deku(id = "104")] NcAsmSetParam(nc_asm::NcAsmSetParam),
    #[deku(id = "105")] NcAsmNtfyParam(nc_asm::NcAsmNtfyParam),

    #[deku(id = "112")] SenseGetCapability(sense::SenseGetCapability),
    #[deku(id = "113")] SenseRetCapability(sense::SenseRetCapability),
    #[deku(id = "116")] SenseSetStatus(sense::SenseSetStatus),

    // Unable to test opt since my WF-1000XM3's dont support it
    #[deku(id = "128")] OptGetCapability(opt::OptGetCapability),
    #[deku(id = "129")] OptRetCapability(opt::OptRetCapability),
    #[deku(id = "130")] OptGetStatus(opt::OptGetStatus),
    #[deku(id = "131")] OptRetStatus(opt::OptRetStatus),
    #[deku(id = "132")] OptSetStatus(opt::OptSetStatus),
    #[deku(id = "133")] OptNtfyStatus(opt::OptNtfyStatus),
    #[deku(id = "134")] OptGetParam(opt::OptGetParam),
    #[deku(id = "135")] OptRetParam(opt::OptRetParam),
    #[deku(id = "137")] OptNtfyParam(opt::OptNtfyParam),

    // Unable to test alert since my WF-1000XM3's dont support it
    #[deku(id = "144")] AlertGetCapability(alert::AlertGetCapability),
    #[deku(id = "145")] AlertRetCapability(alert::AlertRetCapability),
    #[deku(id = "148")] AlertSetStatus(alert::AlertSetStatus),
    #[deku(id = "152")] AlertSetParam(alert::AlertSetParam),
    #[deku(id = "153")] AlertNtfyParam(alert::AlertNtfyParam),

    #[deku(id = "160")] PlayGetCapability(play::PlayGetCapability),
    #[deku(id = "161")] PlayRetCapability(play::PlayRetCapability),
    #[deku(id = "162")] PlayGetStatus(play::PlayGetStatus),
    #[deku(id = "163")] PlayRetStatus(play::PlayRetStatus),
    #[deku(id = "164")] PlaySetStatus(play::PlaySetStatus),
    #[deku(id = "165")] PlayNtfyStatus(play::PlayNtfyStatus),
    #[deku(id = "166")] PlayGetParam(play::PlayGetParam),
    #[deku(id = "167")] PlayRetParam(play::PlayRetParam),
    #[deku(id = "168")] PlaySetParam(play::PlaySetParam),
    #[deku(id = "169")] PlayNtfyParam(play::PlayNtfyParam),

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
