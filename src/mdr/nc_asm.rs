use deku::prelude::*;
use crate::CommonStatus;
use crate::len_str::{read_len_str, write_len_str};
use crate::params::{AsmId, AsmOnOffValue, AsmSettingType, NcAsmEffect, NcAsmInquiredType, NcAsmSettingType, NcDualSingleValue, NcSettingType, NcSettingValue};

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NcAsmGetCapability {
    pub nc_asm_inquired_type: NcAsmInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "NcAsmInquiredType")]
pub enum NcAsmRetCapability {
    #[deku(id = "NcAsmInquiredType::NoiseCancelling")]
    NoiseCancelling(NoiseCancellingRetCapability),

    #[deku(id = "NcAsmInquiredType::NoiseCancellingAndAmbientSoundMode")]
    NoiseCancellingAndAmbientSoundMode(NoiseCancellingAndAmbientRetCapability),

    #[deku(id = "NcAsmInquiredType::AmbientSoundMode")]
    AmbientSoundMode(AmbientSoundModeRetCapability),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NoiseCancellingRetCapability {
    pub nc_setting_type: NcSettingType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NoiseCancellingAndAmbientRetCapability {
    pub nc_setting_type: NcAsmSettingType,
    pub nc_step: u8,
    pub asm_setting_type: AsmSettingType,

    #[deku(update = "self.ambient_sound_modes.len()")]
    pub ambient_sound_mode_count: u8,

    #[deku(count = "ambient_sound_mode_count")]
    pub ambient_sound_modes: Vec<AmbientSoundMode>,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct AmbientSoundModeRetCapability {
    pub asm_setting_type: AsmSettingType,

    #[deku(update = "self.ambient_sound_modes.len()")]
    pub ambient_sound_mode_count: u8,

    #[deku(count = "ambient_sound_mode_count")]
    pub ambient_sound_modes: Vec<AmbientSoundMode>,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct AmbientSoundMode {
    pub asm_id: AsmId,
    pub step: u8,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NcAsmGetStatus {
    pub nc_asm_inquired_type: NcAsmInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NcAsmRetStatus {
    pub nc_asm_inquired_type: NcAsmInquiredType,
    pub status: CommonStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NcAsmNtfyStatus {
    pub nc_asm_inquired_type: NcAsmInquiredType,
    pub status: CommonStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NcAsmGetParam {
    pub nc_asm_inquired_type: NcAsmInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "NcAsmInquiredType")]
pub enum NcAsmRetParam {
    #[deku(id = "NcAsmInquiredType::NoiseCancelling")]
    NoiseCancelling(NoiseCancellingParams),

    #[deku(id = "NcAsmInquiredType::NoiseCancellingAndAmbientSoundMode")]
    NoiseCancellingAndAmbientSoundMode(NoiseCancellingAndAmbientParams),

    #[deku(id = "NcAsmInquiredType::AmbientSoundMode")]
    AmbientSoundMode(AmbientSoundModeParams),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NoiseCancellingParams {
    pub nc_setting_type: NcSettingType,
    pub nc_setting_value: NcSettingValue,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NoiseCancellingAndAmbientParams {
    pub nc_asm_effect: NcAsmEffect,
    pub nc_asm_setting_type: NcAsmSettingType,
    pub persistence: u8, // NcDualSingleValue,

    pub asm_setting_type: AsmSettingType,
    pub asm_id: AsmId,
    pub asm_on_off: u8, // AsmOnOffValue,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct AmbientSoundModeParams {
    pub nc_asm_effect: NcAsmEffect,
    pub asm_setting_type: AsmSettingType,
    pub asm_id: AsmId,
    pub asm_on_off: u8, // AsmOnOffValue,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "NcAsmInquiredType")]
pub enum NcAsmSetParam {
    #[deku(id = "NcAsmInquiredType::NoiseCancelling")]
    NoiseCancelling(NoiseCancellingParams),

    #[deku(id = "NcAsmInquiredType::NoiseCancellingAndAmbientSoundMode")]
    NoiseCancellingAndAmbientSoundMode(NoiseCancellingAndAmbientParams),

    #[deku(id = "NcAsmInquiredType::AmbientSoundMode")]
    AmbientSoundMode(AmbientSoundModeParams),
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "NcAsmInquiredType")]
pub enum NcAsmNtfyParam {
    #[deku(id = "NcAsmInquiredType::NoiseCancelling")]
    NoiseCancelling(NoiseCancellingParams),

    #[deku(id = "NcAsmInquiredType::NoiseCancellingAndAmbientSoundMode")]
    NoiseCancellingAndAmbientSoundMode(NoiseCancellingAndAmbientParams),

    #[deku(id = "NcAsmInquiredType::AmbientSoundMode")]
    AmbientSoundMode(AmbientSoundModeParams),
}
