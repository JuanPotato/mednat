use deku::prelude::*;
use crate::CommonStatus;
use crate::params::{BarometricMeasureType, OptimizerControl, OptimizerInquiredType, OptimizerStatus, PersonalMeasureType};

#[derive(Debug, DekuRead, DekuWrite)]
pub struct OptGetCapability {
    pub optimizer_inquired_type: OptimizerInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct OptRetCapability {
    pub optimizer_inquired_type: OptimizerInquiredType,
    pub optimization_time: u8,
    pub personal_measure_type: PersonalMeasureType,
    pub personal_measure_time: u8,
    pub barometric_measure_type: BarometricMeasureType,
    pub barometric_measure_time: u8,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct OptGetStatus {
    pub optimizer_inquired_type: OptimizerInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct OptRetStatus {
    pub optimizer_inquired_type: OptimizerInquiredType,
    pub status: CommonStatus,
    pub optimizer_status: OptimizerStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct OptSetStatus {
    pub optimizer_inquired_type: OptimizerInquiredType,
    pub status: CommonStatus,
    pub optimizer_control: OptimizerControl,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct OptNtfyStatus {
    pub optimizer_inquired_type: OptimizerInquiredType,
    pub status: CommonStatus,
    pub optimizer_status: OptimizerStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct OptGetParam {
    pub optimizer_inquired_type: OptimizerInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct OptRetParam {
    pub optimizer_inquired_type: OptimizerInquiredType,
    pub personal_measure_type: PersonalMeasureType,
    pub personal_measure_time: u8,
    pub barometric_measure_type: BarometricMeasureType,
    pub barometric_measure_time: u8,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct OptNtfyParam {
    pub optimizer_inquired_type: OptimizerInquiredType,
    pub personal_measure_type: PersonalMeasureType,
    pub personal_measure_time: u8,
    pub barometric_measure_type: BarometricMeasureType,
    pub barometric_measure_time: u8,
}
