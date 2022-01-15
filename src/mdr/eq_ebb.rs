use deku::prelude::*;
use crate::params::{CommonStatus, DisplayLanguage, EqBandInformationType, EqEbbInquiredType, EqPresetId, SpecificInformationType};
use crate::len_str::{read_len_str, write_len_str};

#[derive(Debug, DekuRead, DekuWrite)]
pub struct EqEbbGetCapability {
    pub eq_ebb_inquired_type: EqEbbInquiredType,
    pub display_language: DisplayLanguage,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "EqEbbInquiredType")]
pub enum EqEbbRetCapability {
    #[deku(id = "EqEbbInquiredType::PresetEq")]
    PresetEq(EqRetCapability),

    #[deku(id = "EqEbbInquiredType::Ebb")]
    Ebb(EbbRetCapability),

    #[deku(id = "EqEbbInquiredType::PresetEqNoncustomizable")]
    PresetEqNoncustomizable(EqRetCapability),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct EqRetCapability {
    pub band_count: u8,
    pub level_steps: u8,

    #[deku(update = "self.eq_presets.len()")]
    pub eq_preset_count: u8,

    #[deku(count = "eq_preset_count")]
    pub eq_presets: Vec<PresetEq>,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PresetEq {
    pub id: EqPresetId,
    #[deku(
    reader = "read_len_str(deku::rest)",
    writer = "write_len_str(deku::output, &self.name)"
    )]
    pub name: String, // max len 128
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct EbbRetCapability {
    pub min_ebb_level: i8,
    pub max_ebb_level: i8,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct EqEbbGetStatus {
    pub eq_ebb_inquired_type: EqEbbInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct EqEbbRetStatus {
    pub eq_ebb_inquired_type: EqEbbInquiredType,
    pub status: CommonStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct EqEbbNtfyStatus {
    pub eq_ebb_inquired_type: EqEbbInquiredType,
    pub status: CommonStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct EqEbbGetParam {
    pub eq_ebb_inquired_type: EqEbbInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "EqEbbInquiredType")]
pub enum EqEbbRetParam {
    #[deku(id = "EqEbbInquiredType::PresetEq")]
    PresetEq(PresetEqParam),

    #[deku(id = "EqEbbInquiredType::Ebb")]
    Ebb(EbbParam),

    #[deku(id = "EqEbbInquiredType::PresetEqNoncustomizable")]
    PresetEqNoncustomizable(PresetEqParam),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PresetEqParam {
    pub eq_preset_id: EqPresetId,

    #[deku(update = "self.eq_values.len()")]
    pub eq_values_len: u8,

    #[deku(count = "eq_values_len")]
    pub eq_values: Vec<i8>,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct EbbParam {
    pub ebb_level: i8,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "EqEbbInquiredType")]
pub enum EqEbbSetParam {
    #[deku(id = "EqEbbInquiredType::PresetEq")]
    PresetEq(PresetEqParam),

    #[deku(id = "EqEbbInquiredType::Ebb")]
    Ebb(EbbParam),
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "EqEbbInquiredType")]
pub enum EqEbbNtfyParam {
    #[deku(id = "EqEbbInquiredType::PresetEq")]
    PresetEq(PresetEqParam),

    #[deku(id = "EqEbbInquiredType::Ebb")]
    Ebb(EbbParam),

    #[deku(id = "EqEbbInquiredType::PresetEqNoncustomizable")]
    PresetEqNoncustomizable(PresetEqParam),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct EqEbbGetExtendedInfo {
    pub eq_ebb_inquired_type: EqEbbInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "EqEbbInquiredType")]
pub enum EqEbbRetExtendedInfo {
    #[deku(id = "EqEbbInquiredType::PresetEq")]
    PresetEq(EqRetExtendedInfo),

    #[deku(id = "EqEbbInquiredType::Ebb")]
    Ebb(EqRetExtendedInfo),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct EqRetExtendedInfo {
    #[deku(update = "self.values.len()")]
    pub values_len: u8,

    #[deku(count = "values_len")]
    pub values: Vec<EqValueInfo>,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "EqBandInformationType")]
pub enum EqValueInfo {
    #[deku(id = "EqBandInformationType::NoInformation")]
    NoInformation,

    #[deku(id = "EqBandInformationType::Hz")]
    Hz(Frequency),

    #[deku(id = "EqBandInformationType::Khz")]
    Khz(Frequency),

    #[deku(id = "EqBandInformationType::SpecificInformation")]
    SpecificInformation(SpecificInformationType),
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Frequency {
    val: i16
}
