use crate::params::{CommonCapabilityInquiredType, DeviceInfoInquiredType, FunctionType, GuidanceCategory, ModelSeries, UpdateInquiredType, UpdateMethod};
use crate::len_str::{read_len_str, write_len_str};
use deku::prelude::*;
use crate::CommonStatus;

#[derive(Debug, DekuRead, DekuWrite)]
pub struct SetStatus {
    pub update_inquired_type: UpdateInquiredType,
    pub status: CommonStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct NtfyStatus {
    pub update_inquired_type: UpdateInquiredType,
    pub status: CommonStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetParam {
    pub update_inquired_type: UpdateInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "UpdateInquiredType")]
pub enum RetParam {
    #[deku(id = "UpdateInquiredType::CategoryId")]
    CategoryId(StringRetParam),

    #[deku(id = "UpdateInquiredType::ServiceId")]
    ServiceId(StringRetParam),

    #[deku(id = "UpdateInquiredType::NationCode")]
    NationCode(StringRetParam),

    #[deku(id = "UpdateInquiredType::Language")]
    Language(StringRetParam),

    #[deku(id = "UpdateInquiredType::SerialNumber")]
    SerialNumber(StringRetParam),

    #[deku(id = "UpdateInquiredType::UniqueIdForDeviceBinding")]
    UniqueIdForDeviceBinding(StringRetParam),

    #[deku(id = "UpdateInquiredType::BleTxPower")]
    BleTxPower(BleTxPowerRetParam),

    #[deku(id = "UpdateInquiredType::BatteryPowerThreshold")]
    BatteryPowerThreshold(BatteryPowerThreshold),

    #[deku(id = "UpdateInquiredType::UpdateMethod")]
    UpdateMethod(UpdateMethodRetParam),

    #[deku(id = "UpdateInquiredType::BatteryPowerThresholdForInterruptiongFwUpdate")]
    BatteryPowerThresholdForInterruptiongFwUpdate(BatteryPowerThresholdForInterruptiongFwUpdateRetParam),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct StringRetParam {
    #[deku(
    reader = "read_len_str(deku::rest)",
    writer = "write_len_str(deku::output, &self.str)"
    )]
    pub str: String, // max len 128
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct BleTxPowerRetParam {
    pub tx_power: i8,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct BatteryPowerThreshold {
    pub threshold: u8,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct BatteryPowerThresholdForInterruptiongFwUpdateRetParam {
    pub threshold: u8,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct UpdateMethodRetParam {
    pub update_method: UpdateMethod
}
