use std::array;

use serde::{Deserialize, Serialize};

use crate::util::Result;

use super::super::{Readable, Parser};

#[derive(Deserialize, Serialize)]
pub struct WString {
    s: String,
}

impl WString {
    pub fn empty() -> WString {
        WString { s: String::new() }
    }
}

impl Readable for WString {
    fn read_from(reader: &mut dyn Parser) -> Result<Self>
    where
        Self: Sized,
    {
        let len = reader.read_int()?;
        let mut res: Vec<u16> = Vec::new();
        for _i in 0..len {
            let bytes = array::try_from_fn(|_| reader.read_byte())?;
            res.push(u16::from_le_bytes(bytes));
        }
        Ok(WString {
            s: String::from_utf16(&res).unwrap(),
        })
    }
}
