use dnd_protos::protos::*;

use crate::write_proto;

pub fn generate_skills() {
    write_proto(
        "skills/acrobatics",
        &SkillData {
            name: "acrobatics".to_string(),
            ability: "dexterity".to_string(),
        },
    );

    write_proto(
        "skills/animal_handling",
        &SkillData {
            name: "animal_handling".to_string(),
            ability: "wisdom".to_string(),
        },
    );
    write_proto(
        "skills/arcana",
        &SkillData {
            name: "arcana".to_string(),
            ability: "intelligence".to_string(),
        },
    );
    write_proto(
        "skills/athletics",
        &SkillData {
            name: "athletics".to_string(),
            ability: "strength".to_string(),
        },
    );
    write_proto(
        "skills/deception",
        &SkillData {
            name: "deception".to_string(),
            ability: "charisma".to_string(),
        },
    );
    write_proto(
        "skills/history",
        &SkillData {
            name: "history".to_string(),
            ability: "intelligence".to_string(),
        },
    );
    write_proto(
        "skills/insight",
        &SkillData {
            name: "insight".to_string(),
            ability: "wisdom".to_string(),
        },
    );
    write_proto(
        "skills/intimidation",
        &SkillData {
            name: "intimidation".to_string(),
            ability: "charisma".to_string(),
        },
    );
    write_proto(
        "skills/investigation",
        &SkillData {
            name: "investigation".to_string(),
            ability: "intelligence".to_string(),
        },
    );
    write_proto(
        "skills/medicine",
        &SkillData {
            name: "medicine".to_string(),
            ability: "wisdom".to_string(),
        },
    );
    write_proto(
        "skills/nature",
        &SkillData {
            name: "nature".to_string(),
            ability: "intelligence".to_string(),
        },
    );
    write_proto(
        "skills/perception",
        &SkillData {
            name: "perception".to_string(),
            ability: "wisdom".to_string(),
        },
    );
    write_proto(
        "skills/performance",
        &SkillData {
            name: "performance".to_string(),
            ability: "charisma".to_string(),
        },
    );
    write_proto(
        "skills/persuasion",
        &SkillData {
            name: "persuasion".to_string(),
            ability: "charisma".to_string(),
        },
    );
    write_proto(
        "skills/religion",
        &SkillData {
            name: "religion".to_string(),
            ability: "intelligence".to_string(),
        },
    );
    write_proto(
        "skills/sleight_of_hand",
        &SkillData {
            name: "sleight_of_hand".to_string(),
            ability: "dexterity".to_string(),
        },
    );
    write_proto(
        "skills/stealth",
        &SkillData {
            name: "stealth".to_string(),
            ability: "dexterity".to_string(),
        },
    );
    write_proto(
        "skills/survival",
        &SkillData {
            name: "survival".to_string(),
            ability: "wisdom".to_string(),
        },
    );
}
