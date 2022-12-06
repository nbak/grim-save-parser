use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::parser::{Readable, Parser};
use crate::util::{ensure_eq, Result};

#[derive(Deserialize, Serialize)]
pub struct FormulaSet {
    formulas: HashSet<String>,
}

impl Readable for FormulaSet {
    fn read_from(reader: &mut dyn Parser) -> Result<Self>
    where
        Self: Sized,
    {
        let mut formulas = HashSet::<String>::new();

        ensure_eq(
            String::read_from(reader)?,
            "begin_block".to_owned(),
            "expected begin_block".to_owned(),
        )?;
        reader.read_int()?;
        ensure_eq(
            String::read_from(reader)?,
            "formulasVersion".to_owned(),
            "expected formulasVersion".to_owned(),
        )?;
        let version = reader.read_int()?;
        ensure_eq(
            String::read_from(reader)?,
            "numEntries".to_owned(),
            "expected numEntries".to_owned(),
        )?;
        let num_entries = reader.read_int()?;
        if version >= 3 {
            ensure_eq(
                String::read_from(reader)?,
                "expansionStatus".to_owned(),
                "expected expansionStatus".to_owned(),
            )?;
            reader.read_byte()?;
        }
        for _ in 0..num_entries {
            ensure_eq(
                String::read_from(reader)?,
                "itemName".to_owned(),
                "expected itemName".to_owned(),
            )?;
            formulas.insert(String::read_from(reader)?);
            ensure_eq(
                String::read_from(reader)?,
                "formulaRead".to_owned(),
                "expected formulaRead".to_owned(),
            )?;
            reader.read_int()?;
        }
        ensure_eq(
            String::read_from(reader)?,
            "end_block".to_owned(),
            "expected end_block".to_owned(),
        )?;

        formulas.insert("records/items/crafting/blueprints/relic/craft_relic_b001.dbr".to_string());
        formulas.insert("records/items/crafting/blueprints/relic/craft_relic_b002.dbr".to_string());
        formulas.insert("records/items/crafting/blueprints/relic/craft_relic_b003.dbr".to_string());

        Ok(FormulaSet { formulas })
    }
}
