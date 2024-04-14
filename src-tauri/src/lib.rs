use dnd_protos::{protos::*, CharacterSheetBuilder};
use once_cell::sync::Lazy;
use prost::Message;
use rust_embed::RustEmbed;
use std::{fs, path::PathBuf, sync::RwLock};

#[derive(RustEmbed)]
#[folder = "./generated"]
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
        let builder = CharacterSheetBuilder::new()
            .name("Test")
            .health_system(character_sheet::HealthSystem::Mean(true))
            .class(Class {
                name: "cleric".to_string(),
                subclass: "light".to_string(),
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
        let c = builder.build().unwrap();

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
}
