// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dnd_protos::protos::*;
use dnd_sheet_tauri::{
    calculators::{
        abilities::{calculate, calculate_modifier, calculate_modifier_string, format_modifier},
        classes::get_proficiency_bonus,
        health::get_max_health,
        utils::parse_expression,
    },
    helpers::utils::str_vec_to_string_vec,
    read_proto,
    ui_data::*,
};
use prost::Message;
use std::{collections::HashMap, fs, path::Path, vec};
use tauri::{Manager, State};

#[tauri::command]
fn get_abilities_data(user_data: State<UserData>) -> Vec<AbilitiesDataUI> {
    let mut vec = vec![];
    for ability in &user_data.sheet.abilities {
        let modifier = calculate_modifier_string(&ability.name, &user_data.sheet).unwrap();
        let total = format!("{}", calculate(&ability.name, &user_data.sheet).unwrap());

        let mut proficient = false;
        let full_class_name = format!(
            "classes/{}_{}",
            user_data.sheet.classes.first().unwrap().subclass,
            user_data.sheet.classes.first().unwrap().name
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
    let mut vec = vec![];

    if let Some(skills_data) = read_proto!("skills", SkillsData) {
        for skill_data in skills_data.skills {
            let mut modifier = calculate_modifier(
                skill_data.ability().as_str_name().to_lowercase().as_str(),
                &user_data.sheet,
            )
            .unwrap_or(0);

            if user_data.sheet.skills.contains(&skill_data.name) {
                modifier += get_proficiency_bonus(&user_data.sheet);
            }

            vec.push(SkillDataUI {
                name: skill_data.name.clone(),
                modifier: format_modifier(modifier),
                proficient: user_data.sheet.skills.contains(&skill_data.name),
            })
        }
    }

    vec
}

#[tauri::command]
fn get_counters(user_data: State<UserData>) -> Vec<CounterUI> {
    use dnd_sheet_tauri::calculators::utils::sparse_map_get;

    let mut vec = vec![];

    let classes = &user_data.sheet.classes;

    for class in classes {
        let full_class_name = format!("classes/{}_{}", class.subclass, class.name);
        if let Some(class_data) = read_proto!(full_class_name, ClassData) {
            for counter in class_data.counters {
                let mut max_uses = 0;
                if let Some(stuff) = sparse_map_get(5, &counter.max_uses) {
                    max_uses =
                        parse_expression(stuff, &user_data.sheet).expect("Parsing didn't go well!");
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

    vec
}

struct UserData {
    sheet: CharacterSheet,
}

fn main() {
    let c = CharacterSheet {
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

    println!("level: {}", c.classes.first().unwrap().level);
    println!("max health: {}", get_max_health(&c));

    tauri::Builder::default()
        .setup(move |app| {
            println!("{:#?}", app.path_resolver().app_data_dir().unwrap());
            let data_dir = app.path_resolver().app_data_dir().unwrap().clone();
            let user_data_path = data_dir.as_path().join("user_data");
            if Path::exists(user_data_path.as_path()) {
                println!("Found user data, loading");
                let buf = fs::read(user_data_path).unwrap();
                let sheet = CharacterSheet::decode(buf.as_ref()).unwrap();
                app.manage(UserData { sheet });
            } else {
                println!("Didn't find user data");
                app.manage(UserData { sheet: c.clone() });
                let mut buf = vec![];
                _ = c.encode(&mut buf);
                _ = fs::write(user_data_path, buf);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_abilities_data,
            get_skills_data,
            get_counters
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // let v = dnd_sheet_tauri::Asset::get("light_cleric").unwrap().data;
    // let test = v.as_ref();
    // if let Ok(test) = ClassData::decode(test) {
    //     println!("{}", test.name);
    //     println!("{}", test.hit_die);
    //     if let Some(class_data::CustomProperty::Cleric(c)) = test.custom_property {
    //         println!("{}", c.casting_ability);
    //         if let Some(cleric::Subclass::Light(_l)) = c.subclass {
    //             println!("Found cleric light subclass!");
    //         }
    //     }
    // }

    if let Some(message) = dnd_sheet_tauri::read_proto!("classes/light_cleric", ClassData) {
        println!("Decoded message!");
        println!("{}", message.name);
        println!("{}", message.hit_die);
    } else {
        eprintln!("oops...");
    }

    if let Some(message) = dnd_sheet_tauri::read_proto!("races/godwalker_ra", RaceData) {
        println!("Decoded message!");
        println!("{}", message.name);
        println!("{}", message.size);
    } else {
        eprintln!("oops...");
    }
}
