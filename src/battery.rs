use deku::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum BatteryType {
    #[deku(id = "0")] Battery = 0,
    #[deku(id = "1")] LeftRight = 1,
    #[deku(id = "2")] Cradle = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum BatteryChargingStatus {
    #[deku(id = "0")] NotCharging = 0,
    #[deku(id = "1")] Charging = 1,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct GetBatteryLevel {
    pub battery_type: BatteryType,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum RetBatteryLevel {
    #[deku(id = "0")] Battery(BatteryLevel),
    #[deku(id = "1")] LeftRight(LeftRightBatteryLevel),
    #[deku(id = "2")] Cradle(CradleBatteryLevel),
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum NtfyBatteryLevel {
    #[deku(id = "0")] Battery(BatteryLevel),
    #[deku(id = "1")] LeftRight(LeftRightBatteryLevel),
    #[deku(id = "2")] Cradle(CradleBatteryLevel),
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

