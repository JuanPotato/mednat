use std::num::ParseIntError;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BtAddr(pub [u8; 6]);

impl BtAddr {
    pub fn from_str(addr_str: &str) -> Result<BtAddr, ParseIntError> {
        let mut addr = [0; 6];

        for i in 0..6 {
            addr[i] = u8::from_str_radix(&addr_str[i * 3..i * 3 + 2], 16)?;
        }

        return Ok(BtAddr(addr));
    }

    /// Linux lower-layers actually hold the address in native byte-order
    /// although they are always displayed in network byte-order
    #[inline(always)]
    #[cfg(target_endian = "little")]
    pub fn convert_host_byteorder(mut self) -> BtAddr {
        {
            let (value_1, value_2) = (&mut self.0).split_at_mut(3);
            std::mem::swap(&mut value_1[0], &mut value_2[2]);
            std::mem::swap(&mut value_1[1], &mut value_2[1]);
            std::mem::swap(&mut value_1[2], &mut value_2[0]);
        }

        self
    }

    #[inline(always)]
    #[cfg(target_endian = "big")]
    pub fn convert_host_byteorder(self) -> BtAddr {
        // Public address structure contents are always big-endian
        self
    }
}

impl std::fmt::Display for BtAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}
