use std::array;

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::util::Result;

use super::{
    super::{Parser, Readable},
    HotSlot,
};

#[derive(Deserialize, Serialize)]
pub struct UISettings {
    #[serde(with = "BigArray")]
    hot_slots: [HotSlot; 46],
    unknown4: [String; 5],
    unknown5: [String; 5],
    unknown2: u32,
    camera_distance: f32,
    unknown6: [u8; 5],
    unknown1: u8,
    unknown3: u8,
    maybe_loot_radius: f32,
}

const VERSIONS: [u32; 2] = [5, 6];

impl Readable for UISettings {
    fn read_from(reader: &mut dyn Parser) -> Result<Self> {
        let version = reader.start_block_with_versions(14, &VERSIONS)?;
        let unknown1 = reader.read_byte()?;
        let unknown2 = reader.read_int()?;
        let unknown3 = reader.read_byte()?;
        let mut unknown4 = array::from_fn(|_| String::new());
        let mut unknown5 = array::from_fn(|_| String::new());
        let mut unknown6 = [0; 5];
        for i in 0..5 {
            unknown4[i] = String::read_from(reader)?;
            unknown5[i] = String::read_from(reader)?;
            unknown6[i] = reader.read_byte()?;
        }
        let hot_slots = array::try_from_fn(|_| HotSlot::read_from(reader))?;
        let camera_distance = reader.read_float()?;

        let maybe_loot_radius = if version == 6 {
            reader.read_float()?
        } else {
            -1.0
        };

        reader.end_block()?;

        Ok(UISettings {
            hot_slots,
            unknown4,
            unknown5,
            unknown2,
            camera_distance,
            unknown6,
            unknown1,
            unknown3,
            maybe_loot_radius,
        })
    }
}
