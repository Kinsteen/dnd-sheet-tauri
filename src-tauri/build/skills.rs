use dnd_protos::protos::*;

use crate::write_proto;

pub fn generate_skills() {
    let skills_data = SkillsData {
        skills: vec![
            SkillData {
                name: "acrobatics".to_string(),
                ability: "dexterity".to_string(),
            },
            SkillData {
                name: "animal_handling".to_string(),
                ability: "wisdom".to_string(),
            },
            SkillData {
                name: "arcana".to_string(),
                ability: "intelligence".to_string(),
            },
            SkillData {
                name: "athletics".to_string(),
                ability: "strength".to_string(),
            },
            SkillData {
                name: "deception".to_string(),
                ability: "charisma".to_string(),
            },
            SkillData {
                name: "history".to_string(),
                ability: "intelligence".to_string(),
            },
            SkillData {
                name: "insight".to_string(),
                ability: "wisdom".to_string(),
            },
            SkillData {
                name: "intimidation".to_string(),
                ability: "charisma".to_string(),
            },
            SkillData {
                name: "investigation".to_string(),
                ability: "intelligence".to_string(),
            },
            SkillData {
                name: "medicine".to_string(),
                ability: "wisdom".to_string(),
            },
            SkillData {
                name: "nature".to_string(),
                ability: "intelligence".to_string(),
            },
            SkillData {
                name: "perception".to_string(),
                ability: "wisdom".to_string(),
            },
            SkillData {
                name: "performance".to_string(),
                ability: "charisma".to_string(),
            },
            SkillData {
                name: "persuasion".to_string(),
                ability: "charisma".to_string(),
            },
            SkillData {
                name: "religion".to_string(),
                ability: "intelligence".to_string(),
            },
            SkillData {
                name: "sleight_of_hand".to_string(),
                ability: "dexterity".to_string(),
            },
            SkillData {
                name: "stealth".to_string(),
                ability: "dexterity".to_string(),
            },
            SkillData {
                name: "survival".to_string(),
                ability: "wisdom".to_string(),
            },
        ],
    };

    write_proto("skills", &skills_data);
}
