use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum CommonStatus {
    Enable = 0,
    Disable = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum DeviceInfoInquiredType {
    NoUse = 0,
    ModelName = 1,
    FwVersion = 2,
    SeriesAndColorInfo = 3,
    InstructionGuide = 4,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum EqEbbInquiredType {
    NoUse = 0,
    PresetEq = 1,
    Ebb = 2,
    PresetEqNoncustomizable = 3,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AudioInquiredType {
    NoUse = 0,
    ConnectionMode = 1,
    Upscaling = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum EqBandInformationType {
    NoInformation = 0,
    Hz = 1,
    Khz = 2,
    SpecificInformation = 16,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum UpscalingEffectStatus {
    Off = 0,
    Valid = 1,
    Invalid = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PlayInquiredType {
    NoUse = 0,
    PlaybackController = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum GuidanceCategory {
    ChangeEarpiece = 0,
    WearEarphone = 16,
    PlayButtonOperation = 32,
    TouchPadOperation = 48,
    MainBodyOperation = 64,
    QuickAttention = 80,
    AssignableButtonSettings = 96,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum LinkControlInquiredType {
    KeepAlive = 0,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum BarometricMeasureType {
    NotSupport = 0,
    BarometricPressure = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AlertInquiredType {
    NoUse = 0,
    FixedMessage = 1,
    VibratorAlertNotification = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum BatteryChargingStatus {
    NotCharging = 0,
    Charging = 1,
    Unknown = 240,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum NcSettingType {
    OnOff = 0,
    LevelAdjustment = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ModelSeries {
    NoSeries = 0,
    ExtraBass = 16,
    Hear = 32,
    Premium = 48,
    Sports = 64,
    Casual = 80,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PlaybackStatus {
    Unsettled = 0,
    Play = 1,
    Pause = 2,
    Stop = 3,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum UpscalingSettingType {
    AutoOff = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SportsInquiredType {
    NoUse = 0,
    TrainingMode = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum TrainingModeAvailableEffectType {
    NoUse = 0,
    Type1 = 1,
    Type2 = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AssignableSettingsPreset {
    AmbientSoundControl = 0,
    VolumeControl = 16,
    PlaybackControl = 32,
    VoiceRecognition = 48,
    GoogleAssistant = 49,
    AmazonAlexa = 50,
    TencentXiaowei = 51,
    NoFunction = 255,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AssignableSettingsKey {
    LeftSideKey = 0,
    RightSideKey = 1,
    CustomKey = 2,
    CKey = 3,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AlertVibrationPattern {
    NoUse = 0,
    OnePatternOnly = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SoundPositionType {
    NoUse = 0,
    Type1 = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum NcAsmSettingType {
    OnOff = 0,
    LevelAdjustment = 1,
    DualSingleOff = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SoundPositionPresetId {
    Off = 0,
    FrontLeft = 1,
    FrontRight = 2,
    Front = 3,
    RearLeft = 17,
    RearRight = 18,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum UpscalingSettingValue {
    Off = 0,
    Auto = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum NcSettingValue {
    Off = 0,
    On = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum BarometricPressureValue {
    Unmeasured = 0,
    Measured07 = 7,
    Measured08 = 8,
    Measured09 = 9,
    Measured10 = 10,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum CommonCapabilityInquiredType {
    FixedValue = 0,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum MetaDataDisplayType {
    NotSupport = 0,
    TrackAlbumArtistGenrePlayer = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum FunctionType {
    NoUse = 0,
    BatteryLevel = 17,
    UpscalingIndicator = 18,
    CodecIndicator = 19,
    BleSetup = 20,
    LeftRightBatteryLevel = 21,
    LeftRightConnectionStatus = 23,
    CradleBatteryLevel = 24,
    PowerOff = 33,
    ConciergeData = 34,
    TandemKeepAlive = 35,
    FwUpdate = 48,
    PairingDeviceManagementClassicBt = 56,
    VoiceGuidance = 57,
    Vpt = 65,
    SoundPosition = 66,
    PresetEq = 81,
    Ebb = 82,
    PresetEqNoncustomizable = 83,
    NoiseCancelling = 97,
    NoiseCancellingAndAmbientSoundMode = 98,
    AmbientSoundMode = 99,
    AutoNcAsm = 113,
    NcOptimizer = 129,
    VibratorAlertNotification = 146,
    PlaybackController = 161,
    TrainingMode = 177,
    ActionLogNotifier = 193,
    GeneralSetting1 = 209,
    GeneralSetting2 = 210,
    GeneralSetting3 = 211,
    ConnectionMode = 225,
    Upscaling = 226,
    Vibrator = 241,
    PowerSavingMode = 242,
    ControlByWearing = 243,
    AutoPowerOff = 244,
    SmartTalkingMode = 245,
    AssignableSettings = 246,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ConnectionStatus {
    NotConnected = 0,
    Connected = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AssignableSettingsKeyType {
    TouchSensor = 0,
    Button = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum VptInquiredType {
    NoUse = 0,
    Vpt = 1,
    SoundPosition = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PlaybackNameStatus {
    Unsettled = 0,
    Nothing = 1,
    Settled = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PowerSavingModeSettingValue {
    Off = 0,
    On = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum VibrationType {
    NoPatternSpecified = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AsmSettingType {
    OnOff = 0,
    LevelAdjustment = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PersonalMeasureType {
    NotSupport = 0,
    Personal = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PowerOffInquiredType {
    FixedValue = 0,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PowerSavingModeSettingType {
    OnOff = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AssignableSettingsAction {
    SingleTap = 0,
    DoubleTap = 1,
    TripleTap = 2,
    SingleTapAndHold = 16,
    DoubleTapAndHold = 17,
    LongPressThenActivate = 33,
    LongPressDuringActivation = 34,
    OutOfRange,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum DisplayLanguage {
    UndefinedLanguage = 0,
    English = 1,
    French = 2,
    German = 3,
    Spanish = 4,
    Italian = 5,
    Portuguese = 6,
    Dutch = 7,
    Swedish = 8,
    Finnish = 9,
    Russian = 10,
    Japanese = 11,
    SimplifiedChinese = 12,
    BrazilianPortuguese = 13,
    TraditionalChinese = 14,
    Korean = 15,
    Turkish = 16,
    //CHINESE = -16,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PlaybackControlType {
    NotSupport = 0,
    PlayPauseTrackupTrackdown = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ControlByWearingSettingType {
    OnOff = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AssignableSettingsFunction {
    NoFunction = 0,
    NcAsmOff = 1,
    NcOptimizer = 2,
    QuickAttention = 16,
    VolumeUp = 17,
    VolumeDown = 18,
    PlayPause = 32,
    NextTrack = 33,
    PreviousTrack = 34,
    VoiceRecognition = 48,
    GetYourNotification = 49,
    TalkToGa = 50,
    StopGa = 51,
    VoiceInputCancelAa = 52,
    TalkToTencentXiaowei = 53,
    CancelVoiceRecognition = 54,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ConnectionModeSettingType {
    SoundConnection = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ConnectionStatusInquiredType {
    LeftRightConnectionStatus = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum UpscalingType {
    DseeHx = 0,
    Dsee = 1,
    DseeHxAi = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum BatteryInquiredType {
    Battery = 0,
    LeftRightBattery = 1,
    CradleBattery = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PowerOffSettingValue {
    NoUse = 0,
    UserPowerOff = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SystemInquiredType {
    NoUse = 0,
    Vibrator = 1,
    PowerSavingMode = 2,
    ControlByWearing = 3,
    AutoPowerOff = 4,
    SmartTalkingMode = 5,
    AssignableSettings = 6,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum LogInquiredType {
    NoUse = 0,
    ActionLogNotifier = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum VibratorSettingType {
    OnOff = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum BluetoothDeviceInfoType {
    BluetoothDeviceAddress = 0,
    BleHashValue = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ControlByWearingSettingValue {
    Off = 0,
    On = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum UpscalingEffectType {
    DseeHx = 0,
    Dsee = 1,
    DseeHxAi = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AutoPowerOffParameterType {
    ActiveAndSelectimeId = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SenseInquiredType {
    NoUse = 0,
    AutoNcAsm = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u32")]
pub enum SpecificInformationType {
    ClearBass = 1,
    DontCare = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PersonalValue {
    Unmeasured = 0,
    Measured = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SenseSettingControl {
    NoUse = 0,
    Start = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum EqPresetId {
    Off = 0,
    Rock = 1,
    Pop = 2,
    Jazz = 3,
    Dance = 4,
    Edm = 5,
    RAndBHipHop = 6,
    Acoustic = 7,
    ReservedForFutureNo8 = 8,
    ReservedForFutureNo9 = 9,
    ReservedForFutureNo10 = 10,
    ReservedForFutureNo11 = 11,
    ReservedForFutureNo12 = 12,
    ReservedForFutureNo13 = 13,
    ReservedForFutureNo14 = 14,
    ReservedForFutureNo15 = 15,
    Bright = 16,
    Excited = 17,
    Mellow = 18,
    Relaxed = 19,
    Vocal = 20,
    Treble = 21,
    Bass = 22,
    Speech = 23,
    ReservedForFutureNo24 = 24,
    ReservedForFutureNo25 = 25,
    ReservedForFutureNo26 = 26,
    ReservedForFutureNo27 = 27,
    ReservedForFutureNo28 = 28,
    ReservedForFutureNo29 = 29,
    ReservedForFutureNo30 = 30,
    ReservedForFutureNo31 = 31,
    Custom = 160,
    UserSetting1 = 161,
    UserSetting2 = 162,
    UserSetting3 = 163,
    UserSetting4 = 164,
    UserSetting5 = 165,
    Unspecified,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PlaybackDetailedDataType {
    TrackName = 0,
    AlbumName = 1,
    ArtistName = 2,
    GenreName = 3,
    PlayerName = 16,
    Volume = 32,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum NcAsmEffect {
    Off = 0,
    On = 1,
    AdjustmentInProgress = 16,
    AdjustmentCompletion = 17,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AsmOnOffValue {
    Off = 0,
    On = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum OptimizerStatus {
    Idle = 0,
    InProgressOfPersonal = 1,
    InProgressOfBarometricPressure = 2,
    Optimizing = 16,
    OptimizerEnd = 17,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum CommonOnOffSettingValue {
    Off = 0,
    On = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum CommonOnOffSettingType {
    OnOff = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum NcDualSingleValue {
    Off = 0,
    Single = 1,
    Dual = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AlertAction {
    Negative = 0,
    Positive = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum VptPresetId {
    Off = 0,
    OutdoorFestival = 1,
    Arena = 2,
    ConcertHall = 3,
    Club = 4,
    ReservedForFutureNo5 = 5,
    ReservedForFutureNo6 = 6,
    ReservedForFutureNo7 = 7,
    ReservedForFutureNo8 = 8,
    ReservedForFutureNo9 = 9,
    ReservedForFutureNo10 = 10,
    ReservedForFutureNo11 = 11,
    ReservedForFutureNo12 = 12,
    ReservedForFutureNo13 = 13,
    ReservedForFutureNo14 = 14,
    ReservedForFutureNo15 = 15,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SenseTableType {
    NoUse = 0,
    Type1 = 1,
    Type2 = 2,
    Type3 = 3,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum UpdateInquiredType {
    NoUse = 0,
    FwUpdateMode = 1,
    CategoryId = 2,
    ServiceId = 3,
    NationCode = 4,
    Language = 5,
    SerialNumber = 6,
    BleTxPower = 7,
    BatteryPowerThreshold = 8,
    UpdateMethod = 9,
    BatteryPowerThresholdForInterruptiongFwUpdate = 10,
    UniqueIdForDeviceBinding = 11,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum OptimizerInquiredType {
    NoUse = 0,
    NcOptimizer = 1,
    NcMusicOptimizer = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ConnectionModeSettingValue {
    SoundQualityPrior = 0,
    ConnectionQualityPrior = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum NcOnOffValue {
    Off = 0,
    On = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AlertActionType {
    ConfirmationOnly = 0,
    PositiveNegative = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum VibratorSettingValue {
    Off = 0,
    On = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum OptimizerControl {
    Cancel = 0,
    Start = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum TrainingModeExParameterType {
    NoUse = 0,
    ResetSettings = 1,
    NcasmSettings = 16,
    NcasmActualEffects = 17,
    AsmSettings = 18,
    AsmActualEffects = 19,
    PresetEqSettings = 32,
    PresetEqActualEffects = 33,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PlaybackControl {
    KeyOff = 0,
    Pause = 1,
    TrackUp = 2,
    TrackDown = 3,
    GroupUp = 4,
    GroupDown = 5,
    Stop = 6,
    Play = 7,
    FastForward = 8,
    FastRewind = 9,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AsmId {
    Normal = 0,
    Voice = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AutoPowerOffElementId {
    PowerOffIn5Min = 0,
    PowerOffIn30Min = 1,
    PowerOffIn60Min = 2,
    PowerOffIn180Min = 3,
    PowerOffWhenRemovedFromEars = 16,
    PowerOffDisable = 17,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AlertMessageType {
    NoUse = 0,
    DisconnectCausedByConnectionModeChange = 1,
    DisconnectCausedByChangingKeyAssign = 2,
    NeedDisconnectionForUpdatingFirmware = 3,
    GoogleAssistantIsNowAvailable = 4,
    DualAssignOfVoiceAssistantIsUnavailable = 5,
    DisconnectCausedByChangingMultipointToOn = 6,
    DisconnectCausedByChangingMultipointToOff = 7,
    BatteryConsumptionIncreaseDueToEqAndUpscaling = 8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AudioCodec {
    Unsettled = 0,
    Sbc = 1,
    Aac = 2,
    Ldac = 16,
    AptX = 32,
    AptXHd = 33,
    Other,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum NcAsmInquiredType {
    NoUse = 0,
    NoiseCancelling = 1,
    NoiseCancellingAndAmbientSoundMode = 2,
    AmbientSoundMode = 3,
    OutOfRange,
}

// Generalsetting
#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum GsStringFormat {
    NoUse = 0,
    RawName = 1,
    EnumName = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum GsInquiredType {
    GeneralSetting1 = 209,
    GeneralSetting2 = 210,
    GeneralSetting3 = 211,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum GsSettingType {
    NoUse = 0,
    BooleanType = 1,
    ListType = 2,
    OutOfRange,
}

// Testcommand
#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum AtCommandMessageType {
    Request = 1,
    Response = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum TargetType {
    App = 0,
    HeadphonesOrTwsMaster = 1,
    TwsSlave = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum TestCommandType {
    Atcommand = 5,
    OutOfRange,
}

// Smarttalkingmode
#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SmartTalkingModeEffectStatus {
    NotActive = 0,
    Active = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SmartTalkingModeSettingType {
    OnOff = 0,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SmartTalkingModeDetectionSensitivityType {
    AutoHighLow = 0,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SmartTalkingModeParameterType {
    NoUse = 0,
    ModeOnOff = 1,
    PreviewModeOnOff = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SmartTalkingModeSettingValue {
    Off = 0,
    On = 1,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ModeOutTime {
    Fast = 0,
    Mid = 1,
    Slow = 2,
    None = 3,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SmartTalkingModeVoiceFocusType {
    OnOff = 0,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum DetectionSensitivity {
    Auto = 0,
    High = 1,
    Low = 2,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SmartTalkingModeDetailSettingType {
    Type1 = 0,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SmartTalkingModeModeOutTimeType {
    Type1 = 0,
    OutOfRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum SmartTalkingModePreviewType {
    NotSupport = 0,
    Support = 1,
    OutOfRange,
}
