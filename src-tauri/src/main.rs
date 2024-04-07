// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dnd_protos::protos::*;
use dnd_sheet_tauri::{
    calculators::{
        abilities::{calculate, calculate_modifier, calculate_modifier_string, format_modifier}, classes::get_proficiency_bonus, health::get_max_health, utils::parse_expression
    }, helpers::utils::str_vec_to_string_vec, loaders::{
        disk::{load_sheet, save_current_sheet, save_disk_data, DiskError}, homebrew::{load_in_cache, TEST_STRUCT}, r#static::get_full_class_name
    }, read_class, read_proto, ui_data::*, AppPaths, GeneratedAsset, UserData, APP_STATE
};
use std::{collections::HashMap, path::PathBuf, vec};
use tauri::{AppHandle, Manager};

#[tauri::command]
fn get_abilities_data() -> Vec<AbilitiesDataUI> {
    let state = dnd_sheet_tauri::APP_STATE.read().unwrap();
    if state.is_none() {
        println!("state is none");
        return vec![];
    }

    let unwrapped_state = state.as_ref().unwrap();
    if unwrapped_state.user_data.sheet.is_none() {
        println!("no sheet");
        return vec![];
    }

    let sheet = unwrapped_state.user_data.sheet.as_ref().unwrap();

    let mut vec = vec![];
    for ability in &sheet.abilities {
        let modifier = calculate_modifier_string(&ability.name, sheet).unwrap();
        let total = format!("{}", calculate(&ability.name, sheet).unwrap());

        let mut proficient = false;
        let full_class_name = get_full_class_name(sheet.classes.first().unwrap());
        read_class!([full_class_name.as_str(), class_data] => {
            if class_data.is_some() {
                if class_data.unwrap().saving_throws.contains(&ability.name) {
                    proficient = true;
                }
            } else {
                eprintln!("Didn't find {}", full_class_name);
            }
        });

        vec.push(AbilitiesDataUI {
            name: ability.name.clone(),
            modifier,
            total,
            saving_throw: proficient,
            saving_throw_modifier: "+2".to_string(),
        })
    }

    drop(state);

    vec
}

#[tauri::command]
fn get_skills_data() -> Vec<SkillDataUI> {
    let state = dnd_sheet_tauri::APP_STATE.read().unwrap();
    if state.is_none() {
        return vec![];
    }

    let unwrapped_state = state.as_ref().unwrap();
    if unwrapped_state.user_data.sheet.is_none() {
        return vec![];
    }

    let sheet = unwrapped_state.user_data.sheet.as_ref().unwrap();

    let mut vec = vec![];

    if let Some(skills_data) = read_proto!("skills", SkillsData) {
        for skill_data in skills_data.skills {
            let mut modifier = calculate_modifier(
                skill_data.ability().as_str_name().to_lowercase().as_str(),
                sheet,
            )
            .unwrap_or(0);

            if sheet.skills.contains(&skill_data.name) {
                modifier += get_proficiency_bonus(sheet);
            }

            vec.push(SkillDataUI {
                name: skill_data.name.clone(),
                modifier: format_modifier(modifier),
                proficient: sheet.skills.contains(&skill_data.name),
            })
        }
    }

    vec
}

#[tauri::command]
async fn get_counters() -> Result<Vec<CounterUI>, ()> {
    // let webview_window = tauri::WebviewWindowBuilder::new(
    //     &app,
    //     "label",
    //     tauri::WebviewUrl::App("index.html".into()),
    // )
    // .build()
    // .unwrap();
    // webview_window.set_always_on_top(true);

    use dnd_sheet_tauri::calculators::utils::sparse_map_get;

    let state = dnd_sheet_tauri::APP_STATE.read().unwrap();
    if state.is_none() {
        return Ok(vec![]);
    }

    let unwrapped_state = state.as_ref().unwrap();
    if unwrapped_state.user_data.sheet.is_none() {
        return Ok(vec![]);
    }

    let sheet = unwrapped_state.user_data.sheet.as_ref().unwrap();

    let mut vec = vec![];

    let classes = &sheet.classes;

    for class in classes {
        let full_class_name = get_full_class_name(class);
        read_class!([full_class_name.as_str(), class_data] => {
            if class_data.is_some() {
                for counter in &class_data.unwrap().counters {
                    let mut max_uses = 0;
                    if let Some(stuff) = sparse_map_get(5, &counter.max_uses) {
                        max_uses = parse_expression(stuff, sheet).expect("Parsing didn't go well!");
                    }
                    vec.push(CounterUI {
                        name: counter.name.clone(),
                        used: 0,
                        max_uses: max_uses as i32,
                    });
                }
            } else {
                eprintln!("Didn't find {}", full_class_name);
            }
        });
    }

    Ok(vec)
}

#[tauri::command]
async fn get_health() -> Result<HealthUI, ()> {
    let state = APP_STATE.read().unwrap();
    let state = state.as_ref().unwrap();
    let Some(sheet) = &state.user_data.sheet else {return Err(());};
    
    Ok(HealthUI {
        current: sheet.health,
        max: get_max_health(sheet),
        temporary: 0,
    })
}

#[tauri::command]
async fn get_available_classes() -> Result<Vec<ClassUi>, ()> {
    load_in_cache(); // Load homebrews
    let mut vec = vec![];

    for path in GeneratedAsset::iter() {
        if let Some(name) = path.strip_prefix("classes/") {
            vec.push(ClassUi {
                name: name.to_string()
            });
        }
    }

    let state = TEST_STRUCT.classes.read().unwrap();
    for (name, _data) in state.iter() {
        vec.push(ClassUi {
            name: name.to_string()
        });
    }

    vec.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(vec)
}

#[tauri::command]
async fn get_available_races() -> Result<Vec<ClassUi>, ()> {
    load_in_cache(); // Load homebrews
    let mut vec = vec![];

    for path in GeneratedAsset::iter() {
        if let Some(name) = path.strip_prefix("races/") {
            vec.push(ClassUi {
                name: name.to_string()
            });
        }
    }

    let state = TEST_STRUCT.races.read().unwrap();
    for (name, _data) in state.iter() {
        vec.push(ClassUi {
            name: name.to_string()
        });
    }

    vec.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(vec)
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
