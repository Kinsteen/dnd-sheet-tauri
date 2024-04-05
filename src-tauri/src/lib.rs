use dnd_protos::protos::*;
use helpers::utils::str_vec_to_string_vec;
use once_cell::sync::Lazy;
use prost::Message;
use rust_embed::RustEmbed;
use std::{collections::HashMap, fs, path::PathBuf, sync::RwLock};

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/generated"]
pub struct GeneratedAsset;

pub struct AppPaths {
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
    // TODO error handling
    //  - can't encode for some reason
    //  - can't write
    pub fn save(&self) {
        if let Some(sheet) = &self.sheet {
            println!("Saving user data");
            let mut buf = vec![];
            _ = sheet.encode(&mut buf);
            _ = fs::write(&self.app_paths.sheet_path, buf);
        } else {
            eprintln!("Can't save sheet, nothing loaded");
        }
    }

    pub fn load(&mut self) {
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

        self.sheet = Some(c);
    }
}

pub mod calculators {
    pub mod abilities;
    pub mod classes;
    pub mod health;
    pub mod utils;
}

pub mod helpers {
    pub mod classdata;
    pub mod utils;
}

pub mod loaders {
    pub mod global;
    pub mod homebrew;
    pub mod r#static;
}

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
        pub used: i32,
        pub max_uses: i32,
    }
}
