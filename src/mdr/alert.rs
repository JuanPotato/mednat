use deku::prelude::*;
use crate::CommonStatus;
use crate::params::{AlertAction, AlertActionType, AlertInquiredType, AlertMessageType, AlertVibrationPattern, VibrationType};

#[derive(Debug, DekuRead, DekuWrite)]
pub struct AlertGetCapability {
    pub alert_inquired_type: AlertInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct AlertRetCapability {
    pub alert_inquired_type: AlertInquiredType,
    pub alert_vibration_pattern: AlertVibrationPattern,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct AlertSetStatus {
    pub alert_inquired_type: AlertInquiredType,
    pub status: CommonStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "AlertInquiredType")]
pub enum AlertSetParam {
    #[deku(id = "AlertInquiredType::FixedMessage")]
    FixedMessage(FixedMessageSetParam),

    #[deku(id = "AlertInquiredType::VibratorAlertNotification")]
    VibratorAlertNotification(VibratorAlertNotificationSetParam),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct FixedMessageSetParam {
    pub alert_message_type: AlertMessageType,
    pub alert_action: AlertAction,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct VibratorAlertNotificationSetParam {
    pub vibration_type: VibrationType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct AlertNtfyParam {
    pub alert_inquired_type: AlertInquiredType,
    pub alert_message_type: AlertMessageType,
    pub alert_action_type: AlertActionType,
}
