// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    dnd_sheet_tauri_lib::run();

    // let _c = CharacterSheet {
    //     character_name: "Test".to_string(),
    //     health: 15,
    //     temp_health: None,
    //     health_system: Some(character_sheet::HealthSystem::Mean(true)),
    //     classes: vec![Class {
    //         name: "cleric".to_string(),
    //         subclass: "light".to_string(),
    //         level: 3,
    //         used_cantrips: 0,
    //         spell_slots: vec![],
    //         chosen_skills: vec![],
    //     }],
    //     race: Some(Race {
    //         name: "godwalker_ra".to_string(),
    //     }),
    //     abilities: vec![
    //         Ability {
    //             name: "strength".to_string(),
    //             base_value: 12,
    //         },
    //         Ability {
    //             name: "dexterity".to_string(),
    //             base_value: 12,
    //         },
    //         Ability {
    //             name: "constitution".to_string(),
    //             base_value: 12,
    //         },
    //         Ability {
    //             name: "intelligence".to_string(),
    //             base_value: 13,
    //         },
    //         Ability {
    //             name: "wisdom".to_string(),
    //             base_value: 15,
    //         },
    //         Ability {
    //             name: "charisma".to_string(),
    //             base_value: 9,
    //         },
    //     ],
    //     custom_ability_increases: HashMap::from([("wisdom".to_string(), 1)]),
    //     skills: str_vec_to_string_vec(vec!["history", "medicine"]),
    //     custom_languages: vec![],
    //     counters: vec![],
    // };
}
