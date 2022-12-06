use serde::{Deserialize, Serialize};

use crate::util::{ensure_eq, Result};

use super::{
    super::{Readable, Parser},
    StashTab,
};

#[derive(Deserialize, Serialize)]
pub struct StashFile {
    tabs: Vec<StashTab>,
    some_mod: String,
}

impl Readable for StashFile {
    fn read_from(reader: &mut dyn Parser) -> Result<Self> {
        ensure_eq(reader.read_int()?, 2, "expected to read 2".to_owned())?;

        reader.start_block_with_version(18, 5)?;

        ensure_eq(reader.next_int()?, 0, "expected to read 0".to_owned())?;
        let some_mod = String::read_from(reader)?;
        ensure_eq(reader.read_byte()?, 3, "expected to read 3".to_owned())?;
        let tabs = Vec::read_from(reader)?;

        reader.end_block()?;

        Ok(StashFile { tabs, some_mod })
    }
}
