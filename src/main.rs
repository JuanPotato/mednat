// use std::io::Write;

mod libc_helpe;
mod smol_fd;
mod rfcomm;
mod btaddr;
mod command;

use deku::prelude::*;
use std::io::{Write, BufReader, BufRead};
use command::*;
use crate::btaddr::BtAddr;

fn main() {
    let b = btaddr::BtAddr::from_str("38:18:4C:B6:BF:02").unwrap().convert_host_byteorder();
    let mut client = Client::new(b);

    let get_bat = SonyCommand::GetBatteryLevel(GetBatteryLevel {
        battery_type: BatteryType::LeftRight
    });
    client.send_cmd(&get_bat);

    dbg!(client.read_packet());
    dbg!(client.read_packet().read_cmd());

    let get_bat = SonyCommand::GetBatteryLevel(GetBatteryLevel {
        battery_type: BatteryType::Cradle
    });
    client.send_cmd(&get_bat);

    dbg!(client.read_packet());
    dbg!(client.read_packet().read_cmd());
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

    fn send_packet(&mut self, p: &Packet) {
        println!("Sending packet: {:?} Seqno: {:?} Data: {:x?}", p.data_type, p.seq_no, &p.data);
        let packet_bytes = p.to_bytes().unwrap();
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

