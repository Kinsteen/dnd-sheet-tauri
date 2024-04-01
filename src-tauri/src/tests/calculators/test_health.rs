#[cfg(test)]
#[test]
fn get_max_health_mean_test() {
    use std::collections::HashMap;

    use dnd_protos::protos::*;

    use crate::calculators::health::get_max_health;

    let mut c = CharacterSheet {
        character_name: "Test".to_string(),
        health: 15,
        temp_health: None,
        health_system: Some(character_sheet::HealthSystem::Mean(true)),
        classes: vec![Class {
            name: "cleric".to_string(),
            subclass: "light".to_string(),
            level: 1,
            used_cantrips: 0,
            spell_slots: vec![],
            chosen_skills: vec![],
        }],
        race: None,
        abilities: vec![Ability {
            name: "constitution".to_string(),
            base_value: 12,
        }],
        custom_ability_increases: HashMap::new(),
        skills: vec![],
        custom_languages: vec![],
        counters: vec![],
    };
    assert_eq!(get_max_health(&c), 9);

    c.classes.first_mut().unwrap().level = 2;
    assert_eq!(get_max_health(&c), 15);

    c.classes.first_mut().unwrap().level = 3;
    assert_eq!(get_max_health(&c), 21);

    c.classes.push(Class {
        name: "barbarian".to_string(),
        subclass: String::new(),
        level: 1,
        used_cantrips: 0,
        spell_slots: vec![],
        chosen_skills: vec![],
    });
    assert_eq!(get_max_health(&c), 29);

    c.classes
        .iter_mut()
        .find(|cl| cl.name.eq("barbarian"))
        .unwrap()
        .level = 2;
    assert_eq!(get_max_health(&c), 37);

    c.classes
        .iter_mut()
        .find(|cl| cl.name.eq("cleric"))
        .unwrap()
        .level = 4;
    assert_eq!(get_max_health(&c), 43);

    c.classes.push(Class {
        name: "bard".to_string(),
        subclass: String::new(),
        level: 1,
        used_cantrips: 0,
        spell_slots: vec![],
        chosen_skills: vec![],
    });
    assert_eq!(get_max_health(&c), 49);
}

#[test]
fn get_max_health_rolls_test() {
    use std::collections::HashMap;

    use dnd_protos::protos::*;

    use crate::calculators::health::get_max_health;

    let mut c = CharacterSheet {
        character_name: "Test".to_string(),
        health: 15,
        temp_health: None,
        health_system: Some(character_sheet::HealthSystem::Rolls(HealthRolls {
            rolls: vec![],
        })),
        classes: vec![Class {
            name: "cleric".to_string(),
            subclass: "light".to_string(),
            level: 1,
            used_cantrips: 0,
            spell_slots: vec![],
            chosen_skills: vec![],
        }],
        race: None,
        abilities: vec![Ability {
            name: "constitution".to_string(),
            base_value: 12,
        }],
        custom_ability_increases: HashMap::new(),
        skills: vec![],
        custom_languages: vec![],
        counters: vec![],
    };
    assert_eq!(get_max_health(&c), 9);

    c.classes.first_mut().unwrap().level = 2;
    assert_eq!(get_max_health(&c), 0); // TODO option instead?

    if let character_sheet::HealthSystem::Rolls(rolls) = c.health_system.as_mut().unwrap() {
        rolls.rolls.push(2);
    }
    assert_eq!(get_max_health(&c), 12);

    c.classes.first_mut().unwrap().level = 3;
    assert_eq!(get_max_health(&c), 0);

    if let character_sheet::HealthSystem::Rolls(rolls) = c.health_system.as_mut().unwrap() {
        rolls.rolls.push(8);
    }
    assert_eq!(get_max_health(&c), 21);

    c.classes.push(Class {
        name: "barbarian".to_string(),
        subclass: String::new(),
        level: 1,
        used_cantrips: 0,
        spell_slots: vec![],
        chosen_skills: vec![],
    });
    assert_eq!(get_max_health(&c), 0);

    if let character_sheet::HealthSystem::Rolls(rolls) = c.health_system.as_mut().unwrap() {
        rolls.rolls.push(12);
    }
    assert_eq!(get_max_health(&c), 34);
}
