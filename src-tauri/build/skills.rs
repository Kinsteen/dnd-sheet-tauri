use dnd_protos::protos::*;

use crate::write_proto;

pub fn generate_skills() {
    let skills_data = SkillsData {
        skills: vec![
            SkillData {
                name: "acrobatics".to_string(),
                ability: AbilityEnum::Dexterity.into()
            },
            SkillData {
                name: "animal_handling".to_string(),
                ability: AbilityEnum::Wisdom.into()
            },
            SkillData {
                name: "arcana".to_string(),
                ability: AbilityEnum::Intelligence.into()
            },
            SkillData {
                name: "athletics".to_string(),
                ability: AbilityEnum::Strength.into()
            },
            SkillData {
                name: "deception".to_string(),
                ability: AbilityEnum::Charisma.into()
            },
            SkillData {
                name: "history".to_string(),
                ability: AbilityEnum::Intelligence.into()
            },
            SkillData {
                name: "insight".to_string(),
                ability: AbilityEnum::Wisdom.into()
            },
            SkillData {
                name: "intimidation".to_string(),
                ability: AbilityEnum::Charisma.into()
            },
            SkillData {
                name: "investigation".to_string(),
                ability: AbilityEnum::Intelligence.into()
            },
            SkillData {
                name: "medicine".to_string(),
                ability: AbilityEnum::Wisdom.into()
            },
            SkillData {
                name: "nature".to_string(),
                ability: AbilityEnum::Intelligence.into()
            },
            SkillData {
                name: "perception".to_string(),
                ability: AbilityEnum::Wisdom.into()
            },
            SkillData {
                name: "performance".to_string(),
                ability: AbilityEnum::Charisma.into()
            },
            SkillData {
                name: "persuasion".to_string(),
                ability: AbilityEnum::Charisma.into()
            },
            SkillData {
                name: "religion".to_string(),
                ability: AbilityEnum::Intelligence.into()
            },
            SkillData {
                name: "sleight_of_hand".to_string(),
                ability: AbilityEnum::Dexterity.into()
            },
            SkillData {
                name: "stealth".to_string(),
                ability: AbilityEnum::Dexterity.into()
            },
            SkillData {
                name: "survival".to_string(),
                ability: AbilityEnum::Wisdom.into()
            },
        ]
    };

    write_proto("skills", &skills_data);
}
