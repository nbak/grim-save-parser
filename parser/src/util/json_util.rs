use crate::parser::{Readable, Reader};
use crate::util::{CustomError, Result};
use serde::Serialize;
use std::io::Read;

pub fn map_to_json<T, U>(reader: Box<&mut dyn Reader<Source = T, Item = U>>) -> Result<String>
where
    T: Read,
    U: Readable + Serialize,
{
    let obj = reader.read()?;
    match serde_json::to_string(&obj) {
        Ok(res) => Ok(res),
        Err(e) => Err(CustomError::new(e.to_string())),
    }
}
