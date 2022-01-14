use crate::params::{CommonCapabilityInquiredType, DeviceInfoInquiredType, FunctionType, GuidanceCategory, ModelSeries};
use crate::len_str::{read_len_str, write_len_str};
use deku::prelude::*;

// Protocol Info
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

// Capability Info
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetCapabilityInfo {
    pub capability_inquired: CommonCapabilityInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct RetCapabilityInfo {
    pub capability_inquired: CommonCapabilityInquiredType,
    pub capability_counter: u8,
    #[deku(
    reader = "read_len_str(deku::rest)",
    writer = "write_len_str(deku::output, &self.unique_id)"
    )]
    pub unique_id: String, // max len 128
}

// Device Info
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetDeviceInfo {
    pub info_inquired: DeviceInfoInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "DeviceInfoInquiredType")]
pub enum RetDeviceInfo {
    #[deku(id = "DeviceInfoInquiredType::NoUse")]
    NoUse,

    #[deku(id = "DeviceInfoInquiredType::ModelName")]
    ModelName(DeviceInfoModelName),

    #[deku(id = "DeviceInfoInquiredType::FwVersion")]
    FwVersion(DeviceInfoFwVersion),

    #[deku(id = "DeviceInfoInquiredType::SeriesAndColorInfo")]
    SeriesAndColorInfo(DeviceInfoSeriesAndColorInfo),

    #[deku(id = "DeviceInfoInquiredType::InstructionGuide")]
    InstructionGuide(DeviceInfoInstructionGuide),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct DeviceInfoModelName {
    #[deku(
    reader = "read_len_str(deku::rest)",
    writer = "write_len_str(deku::output, &self.model_name)"
    )]
    pub model_name: String, // max len 128
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct DeviceInfoFwVersion {
    #[deku(
    reader = "read_len_str(deku::rest)",
    writer = "write_len_str(deku::output, &self.fw_version)"
    )]
    pub fw_version: String, // max len 128
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ModelColor {
    Default = 0,
    Black = 1,
    White = 2,
    Silver = 3,
    Red = 4,
    Blue = 5,
    Pink = 6,
    Yellow = 7,
    Green = 8,
    Gray = 9,
    Gold = 10,
    Cream = 11,
    Orange = 12,
    Brown = 13,
    Violet = 14,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct DeviceInfoSeriesAndColorInfo {
    pub series: ModelSeries,
    pub color: ModelColor,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct DeviceInfoInstructionGuide {
    #[deku(update = "self.guidance_categories.len()")]
    pub guidance_len: u8,

    #[deku(count = "guidance_len")]
    pub guidance_categories: Vec<GuidanceCategory>,
}

// Support Function
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetSupportFunction {
    pub common_capability_inquired_type: CommonCapabilityInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct RetSupportFunction {
    pub common_capability_inquired_type: CommonCapabilityInquiredType,

    #[deku(update = "self.support_functions.len()")]
    pub functions_len: u8,

    #[deku(count = "functions_len")]
    pub support_functions: Vec<FunctionType>,
}
