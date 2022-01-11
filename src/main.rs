// use std::io::Write;

mod libc_helpe;
mod smol_fd;
mod rfcomm;
mod btaddr;
mod packet;
mod len_str;
mod params;
mod commands;

use deku::prelude::*;
use std::io::{Write, BufReader, BufRead};
use packet::*;
use crate::btaddr::BtAddr;
use crate::params::{BatteryInquiredType, BluetoothDeviceInfoType, CommonCapabilityInquiredType, CommonStatus, ConnectionStatusInquiredType, DeviceInfoInquiredType, PowerOffInquiredType, PowerOffSettingValue, UpdateInquiredType};
use crate::commands::*;

fn main() {
    let b = btaddr::BtAddr::from_str("94:DB:56:99:AF:26").unwrap().convert_host_byteorder();
    let mut client = Client::new(b);

    let tests = [
        // Connect
        SonyCommand::ConnectGetProtocolInfo(connect::GetProtocolInfo {
            capability_inquired: CommonCapabilityInquiredType::FixedValue,
        }),
        SonyCommand::ConnectGetCapabilityInfo(connect::GetCapabilityInfo {
            capability_inquired: CommonCapabilityInquiredType::FixedValue,
        }),
        SonyCommand::ConnectGetDeviceInfo(connect::GetDeviceInfo {
            info_inquired: DeviceInfoInquiredType::SeriesAndColorInfo,
        }),
        SonyCommand::ConnectGetDeviceInfo(connect::GetDeviceInfo {
            info_inquired: DeviceInfoInquiredType::ModelName,
        }),
        SonyCommand::ConnectGetDeviceInfo(connect::GetDeviceInfo {
            info_inquired: DeviceInfoInquiredType::FwVersion,
        }),
        SonyCommand::ConnectGetDeviceInfo(connect::GetDeviceInfo {
            info_inquired: DeviceInfoInquiredType::InstructionGuide,
        }),
        SonyCommand::ConnectGetSupportFunction(connect::GetSupportFunction {
            common_capability_inquired_type: CommonCapabilityInquiredType::FixedValue
        }),

        // Common
        SonyCommand::CommonGetBatteryLevel(common::GetBatteryLevel {
            battery_type: BatteryInquiredType::LeftRightBattery
        }),
        SonyCommand::CommonGetBatteryLevel(common::GetBatteryLevel {
            battery_type: BatteryInquiredType::CradleBattery
        }),
        SonyCommand::CommonGetUpscalingEffect(common::GetUpscalingEffect {
            common_capability_inquired_type: CommonCapabilityInquiredType::FixedValue
        }),
        SonyCommand::CommonGetAudioCodec(common::GetAudioCodec {
            common_capability_inquired_type: CommonCapabilityInquiredType::FixedValue
        }),
        SonyCommand::CommonGetBluetoothDeviceInfo(common::GetBluetoothDeviceInfo {
            bluetooth_device_info_type: BluetoothDeviceInfoType::BluetoothDeviceAddress
        }),
        SonyCommand::CommonGetBluetoothDeviceInfo(common::GetBluetoothDeviceInfo {
            bluetooth_device_info_type: BluetoothDeviceInfoType::BleHashValue
        }),
        SonyCommand::CommonGetConnectionStatus(common::GetConnectionStatus {
            connection_status_inquired_type: ConnectionStatusInquiredType::LeftRightConnectionStatus
        }),

        // Update
        // SonyCommand::UpdtGetParam(update::GetParam {
        //     update_inquired_type: UpdateInquiredType::NoUse,
        // }),
        // SonyCommand::UpdtGetParam(update::GetParam {
        //     update_inquired_type: UpdateInquiredType::FwUpdateMode,
        // }),
        SonyCommand::UpdtGetParam(update::GetParam {
            update_inquired_type: UpdateInquiredType::CategoryId,
        }),
        SonyCommand::UpdtGetParam(update::GetParam {
            update_inquired_type: UpdateInquiredType::ServiceId,
        }),
        SonyCommand::UpdtGetParam(update::GetParam {
            update_inquired_type: UpdateInquiredType::NationCode,
        }),
        SonyCommand::UpdtGetParam(update::GetParam {
            update_inquired_type: UpdateInquiredType::Language,
        }),
        SonyCommand::UpdtGetParam(update::GetParam {
            update_inquired_type: UpdateInquiredType::SerialNumber,
        }),
        SonyCommand::UpdtGetParam(update::GetParam {
            update_inquired_type: UpdateInquiredType::BleTxPower,
        }),
        SonyCommand::UpdtGetParam(update::GetParam {
            update_inquired_type: UpdateInquiredType::BatteryPowerThreshold,
        }),
        SonyCommand::UpdtGetParam(update::GetParam {
            update_inquired_type: UpdateInquiredType::UpdateMethod,
        }),
        SonyCommand::UpdtGetParam(update::GetParam {
            update_inquired_type: UpdateInquiredType::BatteryPowerThresholdForInterruptiongFwUpdate,
        }),
        SonyCommand::UpdtGetParam(update::GetParam {
            update_inquired_type: UpdateInquiredType::UniqueIdForDeviceBinding,
        }),
    ];

    for test in &tests {
        client.test(test);
    }

    loop {
        let p = dbg!(client.read_packet());
        if p.data_type == DataType::DataMdr {
            dbg!(p.read_cmd());
        }
    }
}


pub struct Client {
    conn: BufReader<rfcomm::RfcommStream>,
    seq_no: u8,
    buffer: Vec<u8>,
}


const FRAME_START_BYTE: u8 = '>' as u8;
const FRAME_END_BYTE: u8 = '<' as u8;

impl Client {
    pub fn new(addr: BtAddr) -> Client {
        let mut conn = rfcomm::RfcommStream::new().unwrap();
        conn.connect(addr.0, 9).unwrap();

        Client {
            conn: BufReader::new(conn),
            seq_no: 0,
            buffer: Vec::with_capacity(1024),
        }
    }

    pub fn send_cmd(&mut self, cmd: &SonyCommand) {
        let mut packet = Packet::new_cmd(cmd);
        packet.seq_no = self.seq_no;
        self.seq_no = 1 - self.seq_no;
        packet.correct_checksum();

        self.send_packet(&packet);
    }

    pub fn test(&mut self, cmd: &SonyCommand) {
        self.send_cmd(cmd);
        let ack = self.read_packet();
        assert!(ack.data_type == DataType::Ack);
        assert!(ack.data.is_empty());

        dbg!(self.read_packet().read_cmd());
    }

    fn send_packet(&mut self, p: &Packet) {
        println!("Sending packet: {:?} Seqno: {:?} Data: {:x?}", p.data_type, p.seq_no, &p.data);
        let packet_bytes = p.to_bytes().unwrap();
        // println!("Write: {:x?}", &packet_bytes);
        let escaped = escape_data(&packet_bytes);

        self.buffer.clear();
        self.buffer.push('>' as u8);

        self.buffer.extend_from_slice(&escaped);

        self.buffer.push('<' as u8);

        self.conn.get_mut().write(&self.buffer).unwrap();
    }

    pub fn read_packet(&mut self) -> Packet {
        self.buffer.clear();
        let len = self.conn.read_until(FRAME_END_BYTE, &mut self.buffer).unwrap();
        // println!("Read: {:x?}", &self.buffer);

        if self.buffer[0] != FRAME_START_BYTE {
            panic!("Unexpected start byte: {:02x}", self.buffer[0]);
        }

        if self.buffer[len - 1] != FRAME_END_BYTE {
            panic!("Unexpected end byte: {:02x}", self.buffer[len - 1]);
        }

        let packet_data = unescape_data(&self.buffer[1..len - 1]);
        let ((remaining, _), p): ((&[u8], _), Packet) = Packet::from_bytes((&packet_data, 0)).unwrap();

        if !remaining.is_empty() {
            panic!("Unexpected leftover Packet bytes. {:?}", remaining);
        }

        if p.checksum != p.calc_checksum() {
            panic!("Incorrect packet checksum.");
        }

        println!("Received packet: {:?} Seqno: {:?} Data: {:x?}", p.data_type, p.seq_no, &p.data);

        if p.needs_ack() {
            self.send_packet(p.get_ack_for());
        }

        return p;
    }
}

fn escape_data(unescaped: &[u8]) -> Vec<u8> {
    let mut escaped = Vec::with_capacity(unescaped.len() * 5 / 4);

    for &byte in unescaped {
        if 60 <= byte && byte <= 62 {
            escaped.push(61);
            escaped.push(byte - 16);
        } else {
            escaped.push(byte);
        }
    }

    escaped
}

fn unescape_data(escaped: &[u8]) -> Vec<u8> {
    let mut unescaped = Vec::with_capacity(escaped.len());
    let mut i = 0;

    while i < escaped.len() {
        if escaped[i] == 61 {
            unescaped.push(escaped[i + 1] + 16);
            i += 2
        } else {
            unescaped.push(escaped[i]);
            i += 1
        }
    }

    unescaped
}

