use serde::{Deserialize, Serialize};

use crate::util::Result;

use super::{
    super::{Readable, Parser},
    ItemSkill, Skill,
};

#[derive(Deserialize, Serialize)]
pub struct CharacterSkills {
    skills: Vec<Skill>,
    item_skills: Vec<ItemSkill>,
    masteries_allowed: u32,
    skill_reclamation_points_used: u32,
    devotion_reclamation_points_used: u32,
}

impl Readable for CharacterSkills {
    fn read_from(reader: &mut dyn Parser) -> Result<Self> {
        reader.start_block_with_version(8, 5)?;

        let skills = Vec::read_from(reader)?;
        let masteries_allowed = reader.read_int()?;
        let skill_reclamation_points_used = reader.read_int()?;
        let devotion_reclamation_points_used = reader.read_int()?;
        let item_skills = Vec::read_from(reader)?;

        reader.end_block()?;

        Ok(CharacterSkills {
            skills,
            item_skills,
            masteries_allowed,
            skill_reclamation_points_used,
            devotion_reclamation_points_used,
        })
    }
}
