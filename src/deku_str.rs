use deku::prelude::*;

pub struct DekuString;

impl DekuString {
    /// Read and convert to String
    pub fn read(
        rest: &BitSlice<Msb0, u8>,
    ) -> Result<(&BitSlice<Msb0, u8>, String), DekuError> {
        let (rest, length) = u8::read(rest, ())?;
        let (rest, data) = Vec::<u8>::read(rest, deku::ctx::Limit::from(length as usize))?;
        let str = String::from_utf8(data)
            .map_err(|e| DekuError::Parse(e.to_string()))?;

        Ok((rest, str))
    }

    /// Parse from String to u8 and write
    pub fn write(output: &mut BitVec<Msb0, u8>, str_field: &str) -> Result<(), DekuError> {
        let value = str_field.len() as u8;
        value.write(output, ())?;

        for v in str_field.as_bytes() {
            v.write(output, ())?;
        }

        Ok(())
    }
}

