use crate::{BluetoothDeviceInfoType, ConnectionStatusInquiredType, PowerOffInquiredType, PowerOffSettingValue};
use crate::params::{AudioCodec, BatteryChargingStatus, BatteryInquiredType, CommonCapabilityInquiredType, CommonStatus, ConnectionStatus, UpscalingEffectStatus, UpscalingEffectType};
use crate::len_str::{read_len_str, write_len_str};
use deku::prelude::*;

// Battery
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetBatteryLevel {
    pub battery_type: BatteryInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "BatteryInquiredType")]
pub enum RetBatteryLevel {
    #[deku(id = "BatteryInquiredType::Battery")]
    Battery(BatteryLevel),

    #[deku(id = "BatteryInquiredType::LeftRightBattery")]
    LeftRight(LeftRightBatteryLevel),

    #[deku(id = "BatteryInquiredType::CradleBattery")]
    Cradle(CradleBatteryLevel),
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "BatteryInquiredType")]
pub enum NtfyBatteryLevel {
    #[deku(id = "BatteryInquiredType::Battery")]
    Battery(BatteryLevel),

    #[deku(id = "BatteryInquiredType::LeftRightBattery")]
    LeftRight(LeftRightBatteryLevel),

    #[deku(id = "BatteryInquiredType::CradleBattery")]
    Cradle(CradleBatteryLevel),
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

// Upscaling
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetUpscalingEffect {
    pub common_capability_inquired_type: CommonCapabilityInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct RetUpscalingEffect {
    pub common_capability_inquired: CommonCapabilityInquiredType,
    pub upscaling_effect_type: UpscalingEffectType,
    pub upscaling_effect_status: UpscalingEffectStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NtfyUpscalingEffect {
    pub common_capability_inquired: CommonCapabilityInquiredType,
    pub upscaling_effect_type: UpscalingEffectType,
    pub upscaling_effect_status: UpscalingEffectStatus,
}

// Audio Codec
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetAudioCodec {
    pub common_capability_inquired_type: CommonCapabilityInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct RetAudioCodec {
    pub common_capability_inquired_type: CommonCapabilityInquiredType,
    pub audio_codec: AudioCodec,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NtfyAudioCodec {
    pub common_capability_inquired_type: CommonCapabilityInquiredType,
    pub audio_codec: AudioCodec,
}

// Bluetooth Device Info
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetBluetoothDeviceInfo {
    pub bluetooth_device_info_type: BluetoothDeviceInfoType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct RetBluetoothDeviceInfo {
    pub bluetooth_device_info_type: BluetoothDeviceInfoType,
    #[deku(
    reader = "read_len_str(deku::rest)",
    writer = "write_len_str(deku::output, &self.bluetooth_info)"
    )]
    pub bluetooth_info: String, // max len 128
}

// Power Off
#[derive(Debug, DekuRead, DekuWrite)]
pub struct SetPowerOff {
    pub power_off_inquired_type: PowerOffInquiredType,
    pub power_off_setting_value: PowerOffSettingValue,
}

// Connection Status
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetConnectionStatus {
    pub connection_status_inquired_type: ConnectionStatusInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "ConnectionStatusInquiredType")]
pub enum RetConnectionStatus {
    #[deku(id = "ConnectionStatusInquiredType::LeftRightConnectionStatus")]
    LeftRight(LeftRightConnectionStatus),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct LeftRightConnectionStatus {
    pub left_status: ConnectionStatus,
    pub right_status: ConnectionStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "ConnectionStatusInquiredType")]
pub enum NtfyConnectionStatus {
    #[deku(id = "ConnectionStatusInquiredType::LeftRightConnectionStatus")]
    LeftRight(LeftRightConnectionStatus),
}

// Concierge Data
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetConciergeData {
    // No clue what this does
    pub common_capability_inquired: CommonCapabilityInquiredType,
    #[deku(count = "deku::rest.len()/8")]
    pub concierge_data: Vec<u8>, // max len 64, no len
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct RetConciergeData {
    pub common_capability_inquired: CommonCapabilityInquiredType,
    #[deku(count = "deku::rest.len()/8")]
    pub concierge_data: Vec<u8>, // max len 1024, no len
}

// Link Control
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "CommonCapabilityInquiredType")]
pub enum SetLinkControl {
    #[deku(id = "CommonCapabilityInquiredType::FixedValue")]
    KeepAlive(KeepAliveLinkControl)
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct KeepAliveLinkControl {
    pub status: CommonStatus,
    pub duration: u8,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "CommonCapabilityInquiredType")]
pub enum NtfyLinkControl {
    #[deku(id = "CommonCapabilityInquiredType::FixedValue")]
    KeepAlive(KeepAliveNtfyLinkControl)
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct KeepAliveNtfyLinkControl {
    pub status: CommonStatus,
}
