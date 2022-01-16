use deku::prelude::*;
use crate::CommonStatus;
use crate::params::{MetaDataDisplayType, PlaybackControl, PlaybackControlType, PlaybackDetailedDataType, PlaybackNameStatus, PlaybackStatus, PlayInquiredType};
use crate::len_str::{read_len_str, write_len_str};

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PlayGetCapability {
    pub play_inquired_type: PlayInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PlayRetCapability {
    pub play_inquired_type: PlayInquiredType,
    pub volume_step: u8,
    pub playback_control_type: PlaybackControlType,
    pub metadata_display_type: MetaDataDisplayType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PlayGetStatus {
    pub play_inquired_type: PlayInquiredType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PlayRetStatus {
    pub play_inquired_type: PlayInquiredType,
    pub status: CommonStatus,
    pub playback_status: PlaybackStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PlaySetStatus {
    pub play_inquired_type: PlayInquiredType,
    pub status: CommonStatus,
    pub playback_control: PlaybackControl,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PlayNtfyStatus {
    pub play_inquired_type: PlayInquiredType,
    pub status: CommonStatus,
    pub playback_status: PlaybackStatus,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PlayGetParam {
    pub play_inquired_type: PlayInquiredType,
    pub playback_detailed_data_type: PlaybackDetailedDataType,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PlayRetParam {
    pub play_inquired_type: PlayInquiredType,
    pub playback_detailed_data: RetParamPlaybackDetailedData,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "PlaybackDetailedDataType")]
pub enum RetParamPlaybackDetailedData {
    #[deku(id = "PlaybackDetailedDataType::TrackName")]
    TrackName(StringPlaybackDetailedData),

    #[deku(id = "PlaybackDetailedDataType::AlbumName")]
    AlbumName(StringPlaybackDetailedData),

    #[deku(id = "PlaybackDetailedDataType::ArtistName")]
    ArtistName(StringPlaybackDetailedData),

    #[deku(id = "PlaybackDetailedDataType::GenreName")]
    GenreName(StringPlaybackDetailedData),

    #[deku(id = "PlaybackDetailedDataType::PlayerName")]
    PlayerName(StringPlaybackDetailedData),

    #[deku(id = "PlaybackDetailedDataType::Volume")]
    Volume(VolumePlaybackDetailedData),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct StringPlaybackDetailedData {
    pub playback_name_status: PlaybackNameStatus,
    #[deku(
    reader = "read_len_str(deku::rest)",
    writer = "write_len_str(deku::output, &self.playback_name)"
    )]
    pub playback_name: String,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct VolumePlaybackDetailedData {
    pub volume: u8,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PlaySetParam {
    pub play_inquired_type: PlayInquiredType,
    pub detailed_data: SetParamPlaybackDetailedData,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "PlaybackDetailedDataType")]
pub enum SetParamPlaybackDetailedData {
    #[deku(id = "PlaybackDetailedDataType::Volume")]
    Volume(VolumePlaybackDetailedData),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PlayNtfyParam {
    pub play_inquired_type: PlayInquiredType,
    pub detailed_data: NtfyParamPlaybackDetailedData,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "PlaybackDetailedDataType")]
pub enum NtfyParamPlaybackDetailedData {
    #[deku(id = "PlaybackDetailedDataType::TrackName")]
    TrackName,

    #[deku(id = "PlaybackDetailedDataType::PlayerName")]
    PlayerName,

    #[deku(id = "PlaybackDetailedDataType::Volume")]
    Volume(VolumePlaybackDetailedData),
}
