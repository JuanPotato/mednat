use crate::smol_fd::SmolFd;
use crate::libc_helpe::libc_check_error;
use libbluetooth::bluetooth::bdaddr_t;
use std::io::{Read, Result, Write};
use std::mem::{size_of, MaybeUninit};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

type RfcommSocketAddr = libbluetooth::rfcomm::sockaddr_rc;

const RFCOMM_ADDR_LEN: usize = size_of::<RfcommSocketAddr>();

pub struct RfcommListener {
    fd: SmolFd,
}

impl RfcommListener {
    pub fn new() -> Result<RfcommListener> {
        let socket = libc_check_error(unsafe {
            libc::socket(
                libc::AF_BLUETOOTH,
                libc::SOCK_STREAM,
                libbluetooth::bluetooth::BTPROTO_RFCOMM,
            )
        })?;

        Ok(RfcommListener {
            fd: SmolFd::new(socket),
        })
    }

    pub fn bind(&self, channel: u8) -> Result<()> {
        let loc_addr = RfcommSocketAddr {
            rc_family: libbluetooth::bluetooth::AF_BLUETOOTH,
            rc_bdaddr: libbluetooth::bluetooth::BDADDR_ANY,
            rc_channel: channel,
        };

        let res = unsafe {
            libc::bind(
                self.fd.raw,
                std::mem::transmute(&loc_addr),
                RFCOMM_ADDR_LEN as u32,
            )
        };

        libc_check_error(res)?;
        Ok(())
    }

    pub fn listen(&self, mode: i32) -> Result<()> {
        let res = unsafe { libc::listen(self.fd.raw, mode) };

        libc_check_error(res)?;
        Ok(())
    }

    pub fn accept(&mut self) -> Result<(RfcommStream, RfcommSocketAddr)> {
        let mut client_addr: MaybeUninit<RfcommSocketAddr> = std::mem::MaybeUninit::uninit();
        let mut client_socklen = RFCOMM_ADDR_LEN as u32;

        let client = unsafe {
            libc::accept(
                self.fd.raw,
                std::mem::transmute(&mut client_addr),
                &mut client_socklen,
            )
        };

        let client_stream = unsafe { RfcommStream::from_raw_fd(libc_check_error(client)?) };
        let client_addr: RfcommSocketAddr = unsafe { client_addr.assume_init() };

        Ok((client_stream, client_addr))
    }
}

impl Read for RfcommListener {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.fd.read(buf)
    }
}

impl Write for RfcommListener {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.fd.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.fd.flush()
    }
}

impl FromRawFd for RfcommListener {
    unsafe fn from_raw_fd(fd: RawFd) -> RfcommListener {
        RfcommListener {
            fd: SmolFd::from_raw_fd(fd),
        }
    }
}

impl AsRawFd for RfcommListener {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

impl Drop for RfcommListener {
    fn drop(&mut self) {
        self.fd.close().unwrap();
    }
}

pub struct RfcommStream {
    fd: SmolFd,
}

impl RfcommStream {
    pub fn new() -> Result<RfcommStream> {
        let socket = libc_check_error(unsafe {
            libc::socket(
                libc::AF_BLUETOOTH,
                libc::SOCK_STREAM,
                libbluetooth::bluetooth::BTPROTO_RFCOMM,
            )
        })?;

        Ok(RfcommStream {
            fd: SmolFd::new(socket),
        })
    }

    pub fn connect(&mut self, bt_addr: [u8; 6], channel: u8) -> Result<()> {
        let loc_addr = RfcommSocketAddr {
            rc_family: libbluetooth::bluetooth::AF_BLUETOOTH,
            rc_bdaddr: bdaddr_t { b: bt_addr },
            rc_channel: channel,
        };

        let res = unsafe {
            libc::connect(
                self.fd.raw,
                std::mem::transmute(&loc_addr),
                RFCOMM_ADDR_LEN as u32,
            )
        };

        libc_check_error(res)?;
        Ok(())
    }
}

impl Read for RfcommStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.fd.read(buf)
    }
}

impl Write for RfcommStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.fd.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.fd.flush()
    }
}

impl FromRawFd for RfcommStream {
    unsafe fn from_raw_fd(fd: RawFd) -> RfcommStream {
        RfcommStream {
            fd: SmolFd::from_raw_fd(fd),
        }
    }
}

impl AsRawFd for RfcommStream {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

impl Drop for RfcommStream {
    fn drop(&mut self) {
        self.fd.close().unwrap();
    }
}
