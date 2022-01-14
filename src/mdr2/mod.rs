use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum Mdr2 {
    #[deku(id = "48")] PeripheralGetCapability,
    #[deku(id = "49")] PeripheralRetCapability,
    #[deku(id = "50")] PeripheralGetStatus,
    #[deku(id = "51")] PeripheralRetStatus,
    #[deku(id = "52")] PeripheralSetStatus,
    #[deku(id = "53")] PeripheralNtfyStatus,
    #[deku(id = "54")] PeripheralGetParam,
    #[deku(id = "55")] PeripheralRetParam,
    #[deku(id = "57")] PeripheralNtfyParam,
    #[deku(id = "60")] PeripheralSetExParam,
    #[deku(id = "61")] PeripheralNtfyExParam,

    #[deku(id = "64")] VoiceGuidanceGetCapability,
    #[deku(id = "65")] VoiceGuidanceRetCapability,
    #[deku(id = "66")] VoiceGuidanceGetStatus,
    #[deku(id = "67")] VoiceGuidanceRetStatus,
    #[deku(id = "68")] VoiceGuidanceSetStatus,
    #[deku(id = "69")] VoiceGuidanceNtfyStatus,
    #[deku(id = "70")] VoiceGuidanceGetParam,
    #[deku(id = "71")] VoiceGuidanceRetParam,
    #[deku(id = "72")] VoiceGuidanceSetParam,
    #[deku(id = "73")] VoiceGuidanceNtfyParam,
}