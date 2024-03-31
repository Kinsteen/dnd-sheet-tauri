// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dnd_protos::protos::classes::*;
use dnd_protos::protos::*;
use dnd_sheet_tauri::{calculators::abilities::calculate, ui_data::*};
use prost::Message;
use std::collections::HashMap;

#[tauri::command]
fn get_abilities_data() -> Vec<AbilitiesDataUI> {
    // TODO load from proto and stuff
    let vec = vec![
        AbilitiesDataUI {
            name: "Strength",
            modifier: "+2",
            total: "12",
            saving_throw: false,
            saving_throw_modifier: "+2",
        },
        AbilitiesDataUI {
            name: "Dexterity",
            modifier: "+2",
            total: "12",
            saving_throw: false,
            saving_throw_modifier: "+2",
        },
        AbilitiesDataUI {
            name: "Constitution",
            modifier: "+2",
            total: "12",
            saving_throw: false,
            saving_throw_modifier: "+2",
        },
        AbilitiesDataUI {
            name: "Intelligence",
            modifier: "+2",
            total: "12",
            saving_throw: false,
            saving_throw_modifier: "+2",
        },
        AbilitiesDataUI {
            name: "Wisdom",
            modifier: "+2",
            total: "12",
            saving_throw: true,
            saving_throw_modifier: "+4",
        },
        AbilitiesDataUI {
            name: "Charisma",
            modifier: "+2",
            total: "12",
            saving_throw: true,
            saving_throw_modifier: "+4",
        },
    ];

    vec
}

#[tauri::command]
fn get_skills_data() -> Vec<SkillDataUI> {
    // TODO load from proto and stuff
    let vec = vec![
        SkillDataUI {
            name: "Acrobatics",
            modifier: "+2",
            proficient: false,
        },
        SkillDataUI {
            name: "azeazeaze",
            modifier: "+4",
            proficient: true,
        },
    ];

    vec
}

fn main() {
    let c = CharacterSheet {
        character_name: "Test".to_string(),
        classes: vec![],
        race: None,
        abilities: vec![Ability {
            name: "dexterity".to_string(),
            base_value: 10,
        }],
        custom_ability_increases: HashMap::new(),
        skills: vec![],
        custom_languages: vec![],
    };

    calculate("dextazeaezerity", &c);
    calculate("dexterity", &c);

    let v = include_bytes!(concat!(env!("OUT_DIR"), "/light_cleric"));

    if let Ok(test) = ClassData::decode(&v[..]) {
        println!("{}", test.name);
        println!("{}", test.hit_die);
        if let Some(class_data::CustomProperty::Cleric(c)) = test.custom_property {
            println!("{}", c.casting_ability);
            if let Some(cleric::Subclass::Light(l)) = c.subclass {
                println!("Found cleric light subclass!");
            }
        }
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_abilities_data,
            get_skills_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
