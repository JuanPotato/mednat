use crate::CommonStatus;
use deku::prelude::*;
use crate::params::{SenseInquiredType, SenseSettingControl, SenseTableType};

#[derive(Debug, DekuRead, DekuWrite)]
pub struct SenseGetCapability {
    pub sense_inquired_type: SenseInquiredType
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct SenseRetCapability {
    pub sense_inquired_type: SenseInquiredType,
    pub sense_table_type: SenseTableType
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct SenseSetStatus {
    pub sense_inquired_type: SenseInquiredType,
    pub status: CommonStatus,
    pub sense_setting_control: SenseSettingControl,
}

