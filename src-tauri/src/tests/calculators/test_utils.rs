#[cfg(test)]
#[test]
fn parse_expression_test() {
    use std::collections::HashMap;

    use dnd_protos::protos::*;

    use crate::{calculators::utils::parse_expression, helpers::utils::str_vec_to_string_vec};

    let sheet = CharacterSheet {
        character_name: "Test".to_string(),
        health: 15,
        temp_health: None,
        health_system: Some(character_sheet::HealthSystem::Mean(true)),
        classes: vec![
            Class {
                name: "cleric".to_string(),
                subclass: "light".to_string(),
                level: 3,
                used_cantrips: 0,
                spell_slots: vec![],
                chosen_skills: vec![],
            },
            Class {
                name: "barbarian".to_string(),
                subclass: "".to_string(),
                level: 2,
                used_cantrips: 0,
                spell_slots: vec![],
                chosen_skills: vec![],
            },
        ],
        race: Some(Race {
            name: "races/godwalker_ra".to_string(),
        }),
        abilities: vec![
            Ability {
                name: "strength".to_string(),
                base_value: 12,
            },
            Ability {
                name: "dexterity".to_string(),
                base_value: 12,
            },
            Ability {
                name: "constitution".to_string(),
                base_value: 12,
            },
            Ability {
                name: "intelligence".to_string(),
                base_value: 13,
            },
            Ability {
                name: "wisdom".to_string(),
                base_value: 15,
            },
            Ability {
                name: "charisma".to_string(),
                base_value: 8,
            },
        ],
        custom_ability_increases: HashMap::from([("wisdom".to_string(), 1)]),
        skills: str_vec_to_string_vec(vec!["history", "medicine"]),
        custom_languages: vec![],
        counters: vec![],
    };

    assert_eq!(parse_expression("1+2", &sheet), Ok(3));
    assert_eq!(parse_expression("4+(5*6)", &sheet), Ok(34));
    assert_eq!(
        parse_expression("test", &sheet),
        Err(evalexpr::EvalexprError::VariableIdentifierNotFound(
            "test".to_string()
        ))
    );
    assert_eq!(parse_expression("str_mod", &sheet), Ok(1));
    assert_eq!(parse_expression("dex_mod", &sheet), Ok(1));
    assert_eq!(parse_expression("con_mod", &sheet), Ok(1));
    assert_eq!(parse_expression("int_mod", &sheet), Ok(1));
    assert_eq!(parse_expression("wis_mod", &sheet), Ok(3));
    assert_eq!(parse_expression("cha_mod", &sheet), Ok(-1));
    assert_eq!(parse_expression("total_level + 2", &sheet), Ok(7));
    assert_eq!(parse_expression("cleric_level + 2", &sheet), Ok(5));
    assert_eq!(parse_expression("barbarian_level", &sheet), Ok(2));
}
