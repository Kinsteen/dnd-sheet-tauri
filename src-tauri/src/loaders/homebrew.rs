use std::{collections::HashMap, fs, path::Path};

use dnd_protos::protos::{BackgroundData, ClassData, RaceData, SkillData};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use prost::Message;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct HomebrewElement<T> {
    pub data: T,
    pub source: String, // Name/ID of the homebrew where the element comes from
}

// Field (name|type) should match field (name|type) in Homebrew struct
pub struct Caches {
    pub classes: RwLock<HashMap<String, HomebrewElement<ClassData>>>,
    pub races: RwLock<HashMap<String, HomebrewElement<RaceData>>>,
    pub skills: RwLock<HashMap<String, HomebrewElement<SkillData>>>,
    pub backgrounds: RwLock<HashMap<String, HomebrewElement<BackgroundData>>>,
}

pub static DATA_CACHE: Lazy<Caches> = Lazy::new(|| Caches {
    classes: RwLock::new(HashMap::new()),
    races: RwLock::new(HashMap::new()),
    skills: RwLock::new(HashMap::new()),
    backgrounds: RwLock::new(HashMap::new()),
});

pub fn load_in_cache() {
    let binding = crate::APP_STATE.read();
    let state = binding.as_ref().unwrap();

    if !Path::exists(state.user_data.app_paths.homebrew_path.as_path()) {
        _ = fs::create_dir_all(state.user_data.app_paths.homebrew_path.as_path());
    }

    let paths = fs::read_dir(state.user_data.app_paths.homebrew_path.as_path()).unwrap();
    drop(binding);

    for path in paths {
        let real_path = path.unwrap().path();

        let data = fs::read(&real_path).unwrap();
        let extension = real_path.extension();
        let homebrew = if extension.is_some() && extension.unwrap().eq("json") {
            let result = serde_json::from_slice(&data);

            if result.is_err() {
                eprintln!("Error loading homebrew {real_path:?}: {:?}", result.err().unwrap());
                continue;
            }

            result.unwrap()
        } else {
            dnd_protos::protos::Homebrew::decode(data.as_ref()).unwrap()
        };

        let homebrew_name = homebrew.name.clone();

        let mut cache = DATA_CACHE.classes.write();
        for class in homebrew.classes {
            cache.insert(class.name.clone(), HomebrewElement {
                data: class,
                source: homebrew_name.clone()
            });
        }
        drop(cache);

        let mut cache = DATA_CACHE.races.write();
        for race in homebrew.races {
            cache.insert(race.name.clone(), HomebrewElement {
                data: race,
                source: homebrew_name.clone()
            });
        }
        drop(cache);

        let mut cache = DATA_CACHE.skills.write();
        for skill in homebrew.skills {
            cache.insert(skill.name.clone(), HomebrewElement {
                data: skill,
                source: homebrew_name.clone()
            });
        }
        drop(cache);

        let mut cache = DATA_CACHE.backgrounds.write();
        for background in homebrew.backgrounds {
            cache.insert(background.name.clone(), HomebrewElement {
                data: background,
                source: homebrew_name.clone()
            });
        }
        drop(cache);
    }
}

/// This macro aims to cache homebrew data, while avoiding full copies of the messages each time.
/// We use a thread safe cache with RwLock (inf readers, one writer)
/// To use this macro:
///
/// read_homebrew! { ClassData, classes ["test", obj_data, wrote] =>
///     println!("{:?}", class_data);
/// }
///
/// Here, "test" is the name/id of the data. obj_data is an identifier available in the block
/// that holds an Option<&"Message Type">>, that we can read. If we need values outside of
/// the macro block, we can copy them, but it's better if we don't to avoid slow copies.
/// There is also the wrote variable, which will be true if we read from the fileystem.
///
/// Every drop(cache) is VERY important, otherwise deadlocks occurs. You can test with
/// the homebrew_test.
#[macro_export]
macro_rules! read_homebrew {
    ($message_type:ident, $field:ident [$name:expr, $classdata:ident] => $($body:tt)*) => {
        let cache = $crate::loaders::homebrew::DATA_CACHE.$field.read();
        if !cache.contains_key($name) {
            // We can arrive here with multiple threads if we're unlucky. Should not be a problem,
            // but may do unnecessary fs read.

            // Drop read cache, reopen write cache, write, then close
            drop(cache);

            $crate::loaders::homebrew::load_in_cache();
        } else {
            drop(cache);
        }

        // Cache is always dropped before
        let cache = $crate::loaders::homebrew::DATA_CACHE.$field.read();
        let $classdata = if cache.get($name).is_some() {
            Some(&cache.get($name).unwrap().data)
        } else {
            None
        };
        $($body)*
        drop(cache);
    };
}
