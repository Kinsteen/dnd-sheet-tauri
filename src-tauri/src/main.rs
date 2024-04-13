// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dnd_protos::{protos::*, CharacterSheetBuilder};
use dnd_sheet_tauri::{
    commands::getters::*,
    helpers::utils::str_vec_to_string_vec,
    loaders::disk::{load_sheet, save_current_sheet, save_disk_data, DiskError},
    read_class, read_race,
    AppPaths, UserData,
};
use std::{collections::HashMap, path::PathBuf, vec};
use tauri::Manager;

#[tauri::command]
async fn create_sheet(
    character_name: String,
    class: String,
    race: String,
    health_system_mean: bool,
    abilities: HashMap<String, i32>,
    skills: Vec<String>,
) -> Result<(), String> {
    read_class!([&class, class_data] => {
        if class_data.is_none() {
            eprintln!("Class doesn't exists!");
            return Err("Class doesn't exists!".to_string());
        }
    });

    read_race!([&race, race_data] => {
        if race_data.is_none() {
            eprintln!("Race doesn't exists!");
            return Err("Race doesn't exists!".to_string());
        }
    });

    let mut builder = CharacterSheetBuilder::new();
    builder = builder.name(character_name);
    builder = builder.class(Class {
        name: class,
        subclass: "".to_owned(),
        level: 1,
        used_cantrips: 0,
        spell_slots: vec![],
        chosen_skills: vec![], // TODO
    });

    builder = builder.race(Race { name: race });

    if health_system_mean {
        builder = builder.health_system(character_sheet::HealthSystem::Mean(true));
    } else {
        builder = builder.health_system(character_sheet::HealthSystem::Rolls(HealthRolls {
            rolls: vec![],
        }));
    }

    for (ability, score) in abilities.iter() {
        builder = builder.ability(ability, score.to_owned());
    }

    for skill in skills {
        builder = builder.skill(skill);
    }

    let sheet = builder.build();
    println!("{sheet:#?}");

    Ok(())
}

fn setup_user_data(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(dev) {
        println!("Running in dev mode.");
        let mut state = dnd_sheet_tauri::APP_STATE.write().unwrap();
        *state = Some(dnd_sheet_tauri::State {
            user_data: dnd_sheet_tauri::UserData {
                sheet: None,
                app_paths: dnd_sheet_tauri::AppPaths {
                    user_data_path: PathBuf::from("./dev_data/saved_data"),
                    sheet_path: PathBuf::from("./dev_data/sheets/"),
                    homebrew_path: PathBuf::from("./src/tests/resources/homebrew/"),
                },
            },
        });
        drop(state);

        let disk_data = load_sheet();
        match disk_data {
            Err(DiskError::DecodeError) => {
                eprintln!("Error decoding the sheet");
            }
            Err(DiskError::FileNotFound) => {
                // TODO temp
                eprintln!("Default sheet not found");
                let mut state = dnd_sheet_tauri::APP_STATE.write().unwrap();
                state.as_mut().unwrap().user_data.load();
                drop(state);
                _ = save_disk_data();
                _ = save_current_sheet();
            }
            Err(DiskError::NoState) => {
                println!("No way")
            }
            Ok(sheet) => {
                let mut state = dnd_sheet_tauri::APP_STATE.write().unwrap();
                state.as_mut().unwrap().user_data.sheet = Some(sheet);
                drop(state);
            }
        }
    } else if let Ok(root_path) = app.path().app_data_dir() {
        let app_paths = AppPaths {
            user_data_path: root_path.join("user_data"),
            sheet_path: root_path.join("sheets/"),
            homebrew_path: root_path.join("homebrew/"),
        };

        let mut user_data = UserData {
            sheet: None,
            app_paths: app_paths.clone(),
        };
        user_data.load();

        let mut state = dnd_sheet_tauri::APP_STATE.write().unwrap();
        *state = Some(dnd_sheet_tauri::State { user_data });
    } else {
        panic!("Can't find app data dir, exiting");
    }

    Ok(())
}

fn main() {
    let _c = CharacterSheet {
        character_name: "Test".to_string(),
        health: 15,
        temp_health: None,
        health_system: Some(character_sheet::HealthSystem::Mean(true)),
        classes: vec![Class {
            name: "cleric".to_string(),
            subclass: "light".to_string(),
            level: 3,
            used_cantrips: 0,
            spell_slots: vec![],
            chosen_skills: vec![],
        }],
        race: Some(Race {
            name: "godwalker_ra".to_string(),
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
                base_value: 9,
            },
        ],
        custom_ability_increases: HashMap::from([("wisdom".to_string(), 1)]),
        skills: str_vec_to_string_vec(vec!["history", "medicine"]),
        custom_languages: vec![],
        counters: vec![],
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(setup_user_data)
        .invoke_handler(tauri::generate_handler![
            get_abilities_data,
            get_skills_data,
            get_counters,
            get_health,
            get_available_classes,
            get_available_races,
            create_sheet,
            get_available_skills,
            calculate_ability,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
