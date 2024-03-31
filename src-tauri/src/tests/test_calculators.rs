#[cfg(test)]
use std::collections::HashMap;

#[test]
fn calculate_abilities_test() {
    use crate::calculators::abilities::calculate;
    use dnd_protos::protos::*;

    let sheet_no_race = CharacterSheet {
        character_name: "Test".to_string(),
        classes: vec![],
        race: None,
        abilities: vec![
            Ability {
                name: "dexterity".to_string(),
                base_value: 10,
            },
            Ability {
                name: "wisdom".to_string(),
                base_value: 15,
            },
        ],
        custom_ability_increases: HashMap::new(),
        skills: vec![],
        custom_languages: vec![],
    };

    assert_eq!(calculate("dexterity", &sheet_no_race), Some(10));
    assert_eq!(calculate("wisdom", &sheet_no_race), Some(15));

    let mut sheet_with_race = sheet_no_race.clone();
    sheet_with_race.race = Some(Race {
        name: "godwalker_ra".to_string(),
    });

    assert_eq!(calculate("dexterity", &sheet_with_race), Some(10));
    assert_eq!(calculate("wisdom", &sheet_with_race), Some(17));

    let mut sheet_with_custom = sheet_with_race.clone();
    sheet_with_custom
        .custom_ability_increases
        .insert("wisdom".to_string(), 1);

    assert_eq!(calculate("dexterity", &sheet_with_custom), Some(10));
    assert_eq!(calculate("wisdom", &sheet_with_custom), Some(18));
}

#[test]
fn calculate_modifier_string_test() {
    use crate::calculators::abilities::calculate_modifier_string;
    use dnd_protos::protos::*;

    let sheet_no_race = CharacterSheet {
        character_name: "Test".to_string(),
        classes: vec![],
        race: None,
        abilities: vec![
            Ability {
                name: "dexterity".to_string(),
                base_value: 10,
            },
            Ability {
                name: "wisdom".to_string(),
                base_value: 15,
            },
        ],
        custom_ability_increases: HashMap::new(),
        skills: vec![],
        custom_languages: vec![],
    };

    assert_eq!(
        calculate_modifier_string("dexterity", &sheet_no_race),
        Some("+0".to_string())
    );
    assert_eq!(
        calculate_modifier_string("wisdom", &sheet_no_race),
        Some("+2".to_string())
    );

    let mut sheet_with_race = sheet_no_race.clone();
    sheet_with_race.race = Some(Race {
        name: "godwalker_ra".to_string(),
    });

    assert_eq!(
        calculate_modifier_string("dexterity", &sheet_with_race),
        Some("+0".to_string())
    );
    assert_eq!(
        calculate_modifier_string("wisdom", &sheet_with_race),
        Some("+3".to_string())
    );

    let mut sheet_with_custom = sheet_with_race.clone();
    sheet_with_custom
        .custom_ability_increases
        .insert("wisdom".to_string(), 1);

    assert_eq!(
        calculate_modifier_string("dexterity", &sheet_with_custom),
        Some("+0".to_string())
    );
    assert_eq!(
        calculate_modifier_string("wisdom", &sheet_with_custom),
        Some("+4".to_string())
    );

    // TODO more tests
}
