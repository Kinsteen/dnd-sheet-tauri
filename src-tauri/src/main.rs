// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dnd_protos::protos::*;
use dnd_sheet_tauri::{
    calculators::{
        abilities::{calculate, calculate_modifier, calculate_modifier_string, format_modifier},
        classes::get_proficiency_bonus,
        utils::parse_expression,
    },
    helpers::utils::str_vec_to_string_vec,
    read_proto,
    ui_data::*,
    AppPaths, UserData,
};
use std::{collections::HashMap, vec};
use tauri::{AppHandle, Manager, State};

#[tauri::command]
fn get_abilities_data(user_data: State<UserData>) -> Vec<AbilitiesDataUI> {
    if user_data.sheet.is_none() {
        return vec![];
    }

    let sheet = user_data.sheet.as_ref().unwrap();

    let mut vec = vec![];
    for ability in &sheet.abilities {
        let modifier = calculate_modifier_string(&ability.name, sheet).unwrap();
        let total = format!("{}", calculate(&ability.name, sheet).unwrap());

        let mut proficient = false;
        let full_class_name = format!(
            "classes/{}_{}",
            sheet.classes.first().unwrap().subclass,
            sheet.classes.first().unwrap().name
        );
        if let Some(class_data) = read_proto!(full_class_name, ClassData) {
            if class_data.saving_throws.contains(&ability.name) {
                proficient = true;
            }
        } else {
            eprintln!("Didn't find {}", full_class_name);
        }

        vec.push(AbilitiesDataUI {
            name: ability.name.clone(),
            modifier,
            total,
            saving_throw: proficient,
            saving_throw_modifier: "+2".to_string(),
        })
    }

    vec
}

#[tauri::command]
fn get_skills_data(user_data: State<UserData>) -> Vec<SkillDataUI> {
    if user_data.sheet.is_none() {
        return vec![];
    }

    let sheet = user_data.sheet.as_ref().unwrap();

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
async fn get_counters(
    user_data: State<'_, UserData>,
    app: AppHandle,
) -> Result<Vec<CounterUI>, ()> {
    // let webview_window = tauri::WebviewWindowBuilder::new(
    //     &app,
    //     "label",
    //     tauri::WebviewUrl::App("index.html".into()),
    // )
    // .build()
    // .unwrap();
    // webview_window.set_always_on_top(true);

    use dnd_sheet_tauri::calculators::utils::sparse_map_get;

    if user_data.sheet.is_none() {
        return Ok(vec![]);
    }

    let sheet = user_data.sheet.as_ref().unwrap();

    let mut vec = vec![];

    let classes = &sheet.classes;

    for class in classes {
        let full_class_name = format!("classes/{}_{}", class.subclass, class.name);
        if let Some(class_data) = read_proto!(full_class_name, ClassData) {
            for counter in class_data.counters {
                let mut max_uses = 0;
                if let Some(stuff) = sparse_map_get(5, &counter.max_uses) {
                    max_uses = parse_expression(stuff, sheet).expect("Parsing didn't go well!");
                }
                vec.push(CounterUI {
                    name: counter.name,
                    used: 0,
                    max_uses: max_uses as i32,
                });
            }
        } else {
            eprintln!("Didn't find {}", full_class_name);
        }
    }

    Ok(vec)
}

fn setup_user_data(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(root_path) = app.path().app_data_dir() {
        let app_paths = AppPaths {
            sheet_path: root_path.join("sheet"),
            homebrew_path: root_path.join("homebrew/"),
        };

        let mut user_data = UserData {
            sheet: None,
            app_paths,
        };
        user_data.load();
        app.manage(user_data);
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
            get_counters
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
