use crate::calculators::health::get_max_health;
use crate::commands::getters::*;
use crate::helpers::sheet_builder::CharacterSheetBuilder;
use crate::loaders::disk::*;
use dnd_protos::protos::*;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::Manager;

#[derive(RustEmbed)]
#[folder = "./generated"]
#[exclude = "tests/*"]
pub struct GeneratedAsset;

#[derive(Clone)]
pub struct AppPaths {
    pub user_data_path: PathBuf,
    pub sheet_path: PathBuf,
    pub homebrew_path: PathBuf,
}

pub struct UserData {
    pub sheet: Option<CharacterSheet>,
    pub app_paths: AppPaths,
}

// I don't think that's good... But it works
pub static APP_STATE: Lazy<RwLock<Option<State>>> = Lazy::new(|| RwLock::new(None));

pub struct State {
    pub user_data: UserData,
}

impl UserData {
    pub fn load(&mut self) {
        println!("Loading default sheet");
        let builder = CharacterSheetBuilder::new()
            .name("Test")
            .health_system(character_sheet::HealthSystem::Mean(true))
            .class(Class {
                name: "life_cleric".to_string(),
                subclass: "".to_string(),
                level: 3,
                used_cantrips: 0,
                spell_slots: vec![],
                chosen_skills: vec![],
            })
            .race(Race {
                name: "tiefling".to_string(),
            })
            .ability("strength", 12)
            .ability("dexterity", 12)
            .ability("constitution", 12)
            .ability("intelligence", 13)
            .ability("wisdom", 15)
            .ability("charisma", 9)
            .custom_ability_increase("wisdom", 1)
            .skill("history")
            .skill("medicine");
        let mut c = builder.build().unwrap();
        c.health = get_max_health(&c);
        self.sheet = Some(c);
    }
}

pub mod calculators {
    pub mod abilities;
    pub mod classes;
    pub mod health;
    pub mod utils;
}

pub mod commands {
    pub mod getters;
}

pub mod helpers {
    pub mod classdata;
    pub mod sheet_builder;
    pub mod utils;
}

pub mod loaders {
    pub mod disk;
    pub mod global;
    pub mod homebrew;
    pub mod r#static;
}

#[cfg(test)]
mod tests {
    mod calculators {
        mod test_abilities;
        mod test_classes;
        mod test_health;
        mod test_utils;
    }

    mod loaders {
        mod test_homebrew;
    }

    mod test_proto_helpers;
}

pub mod ui_data {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct AbilitiesDataUI {
        pub name: String,
        pub modifier: String,
        pub total: String,
        pub saving_throw: bool,
        pub saving_throw_modifier: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SkillDataUI {
        pub name: String,
        pub modifier: String,
        pub proficient: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CounterUI {
        pub name: String,
        pub class: String, // Used for i18n
        pub used: i32,
        pub max_uses: i32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct HealthUI {
        pub current: i32,
        pub max: i32,
        pub temporary: i32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ClassUi {
        pub name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SkillsUI {
        pub skills: Vec<String>,
        pub num_to_pick: i32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct BasicDataClassUI {
        pub name: String,
        pub level: i32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct BasicDataUI {
        pub character_name: String,
        pub classes: Vec<BasicDataClassUI>,
        pub race: String,
        pub total_level: i32,
    }
}

#[tauri::command]
async fn create_sheet(
    app: tauri::AppHandle,
    character_name: String,
    class: String,
    race: String,
    health_system_mean: bool,
    abilities: HashMap<String, i32>,
    skills: Vec<String>,
) -> Result<(), String> {
    // read_class!([&class, class_data] => {
    //     if class_data.is_none() {
    //         eprintln!("Class doesn't exists!");
    //         return Err("Class doesn't exists!".to_string());
    //     }
    // });

    // read_race!([&race, race_data] => {
    //     if race_data.is_none() {
    //         eprintln!("Race doesn't exists!");
    //         return Err("Race doesn't exists!".to_string());
    //     }
    // });

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

    builder.can_build()?;

    let sheet = builder.build().unwrap();

    save_sheet_to_disk(&sheet).unwrap(); // TODO error

    {
        let mut state = crate::APP_STATE.write();
        let _ = state
            .as_mut()
            .unwrap()
            .user_data
            .sheet
            .insert(sheet.clone());
    }

    // Saves updated user data
    save_current_user_data().unwrap();

    app.emit("reload-sheet", ()).unwrap();
    Ok(())
}

#[tauri::command]
async fn change_health(value: i32) -> Result<(), String> {
    let mut state = crate::APP_STATE.write();
    if let Some(sheet) = state.as_mut().unwrap().user_data.sheet.as_mut() {
        sheet.health += value;
    }
    drop(state);
    save_current_sheet().unwrap();
    Ok(())
}

#[tauri::command]
async fn change_counter(name: String, value: i32) -> Result<(), String> {
    let mut state = crate::APP_STATE.write();
    if let Some(sheet) = state.as_mut().unwrap().user_data.sheet.as_mut() {
        if let Some(counter) = sheet.counters.iter_mut().find(|c| c.name.eq(&name)) {
            counter.used += value;
        }
    }
    drop(state);
    save_current_sheet().unwrap();
    Ok(())
}

#[tauri::command]
async fn change_sheet(name: String, app: tauri::AppHandle) -> Result<(), String> {
    let mut state = crate::APP_STATE.upgradable_read();
    let sheet_path = state
        .as_ref()
        .unwrap()
        .user_data
        .app_paths
        .sheet_path
        .join(name);
    let sheet = load_sheet_from_path(sheet_path.as_path());

    if sheet.is_ok() {
        let sheet = sheet.unwrap();
        state.with_upgraded(|s| {
            let s = s.as_mut().unwrap();
            let _ = s.user_data.sheet.insert(sheet);
        });

        app.emit("reload-sheet", ()).unwrap();
    } else {
        panic!("Couldn't load sheet!");
    }

    Ok(())
}

fn setup_user_data(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    if false {
        println!("Running in dev mode.");
        let mut state = crate::APP_STATE.write();
        *state = Some(crate::State {
            user_data: crate::UserData {
                sheet: None,
                app_paths: crate::AppPaths {
                    user_data_path: PathBuf::from("./dev_data/saved_data"),
                    sheet_path: PathBuf::from("./dev_data/sheets/"),
                    homebrew_path: PathBuf::from("./src/tests/resources/homebrew/"),
                },
            },
        });
        drop(state);
    } else if let Ok(root_path) = app.path().app_data_dir() {
        let mut state = crate::APP_STATE.write();
        *state = Some(crate::State {
            user_data: crate::UserData {
                sheet: None,
                app_paths: crate::AppPaths {
                    user_data_path: root_path.join("user_data"),
                    sheet_path: root_path.join("sheets/"),
                    homebrew_path: root_path.join("homebrew/"),
                },
            },
        });
        drop(state);
    } else {
        panic!("Can't find app data dir, exiting");
    }

    let disk_data = load_current_sheet();
    match disk_data {
        Err(DiskError::DecodeError) => {
            eprintln!("Error decoding the sheet");
        }
        Err(DiskError::FileNotFound) => {
            // TODO temp
            eprintln!("Default sheet not found");
            let mut state = crate::APP_STATE.write();
            state.as_mut().unwrap().user_data.load();
            drop(state);
            _ = save_current_user_data();
            _ = save_current_sheet();
        }
        Err(DiskError::NoState) => {
            println!("No state found, app loaded very badly");
        }
        Ok(sheet) => {
            let mut state = crate::APP_STATE.write();
            state.as_mut().unwrap().user_data.sheet = Some(sheet);
            drop(state);
        }
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(setup_user_data)
        .invoke_handler(tauri::generate_handler![
            get_abilities_data,
            get_skills_data,
            get_counters,
            get_health,
            get_available_classes,
            get_available_races,
            get_available_backgrounds,
            create_sheet,
            get_available_skills,
            calculate_ability,
            change_health,
            change_counter,
            get_basic_data,
            get_sheets,
            change_sheet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
