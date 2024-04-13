use std::{collections::HashMap, fs, sync::RwLock};

use prost::Message;
use dnd_protos::protos::{ClassData, RaceData, SkillData, BackgroundData};
use once_cell::sync::Lazy;

// Field name should match field name in Homebrew struct
// Field type should match field type in Homebrew struct
pub struct Caches {
    pub classes: RwLock<HashMap<String, ClassData>>,
    pub races: RwLock<HashMap<String, RaceData>>,
    pub skills: RwLock<HashMap<String, SkillData>>,
    pub backgrounds: RwLock<HashMap<String, BackgroundData>>
}

pub static DATA_CACHE: Lazy<Caches> = Lazy::new(|| Caches {
    classes: RwLock::new(HashMap::new()),
    races: RwLock::new(HashMap::new()),
    skills: RwLock::new(HashMap::new()),
    backgrounds: RwLock::new(HashMap::new()),
});

pub fn load_in_cache() {
    let binding = crate::APP_STATE.read().unwrap();
    let state = binding.as_ref().unwrap();
    let paths = fs::read_dir(state.user_data.app_paths.homebrew_path.as_path()).unwrap();
    drop(binding);

    for path in paths {
        let real_path = path.unwrap().path();
        let data = fs::read(real_path).unwrap();
        let homebrew = dnd_protos::protos::Homebrew::decode(data.as_ref()).unwrap();

        let mut cache = DATA_CACHE.classes.write().unwrap();
        for class in homebrew.classes {
            cache.insert(class.name.clone(), class);
        }
        drop(cache);

        let mut cache = DATA_CACHE.races.write().unwrap();
        for race in homebrew.races {
            cache.insert(race.name.clone(), race);
        }
        drop(cache);

        let mut cache = DATA_CACHE.skills.write().unwrap();
        for skill in homebrew.skills {
            cache.insert(skill.name.clone(), skill);
        }
        drop(cache);

        let mut cache = DATA_CACHE.backgrounds.write().unwrap();
        for background in homebrew.backgrounds {
            cache.insert(background.name.clone(), background);
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
    ($message_type:ident, $field:ident [$name:expr, $classdata:ident, $wrote:ident] => $($body:tt)*) => {
        let mut $wrote = false;
        let cache = $crate::loaders::homebrew::DATA_CACHE.$field.read().unwrap();
        if !cache.contains_key($name) {
            // We can arrive here with multiple threads if we're unlucky. Should not be a problem,
            // but may do unnecessary fs read.

            // Drop read cache, reopen write cache, write, then close
            drop(cache);

            $crate::loaders::homebrew::load_in_cache();
        } else {
            drop(cache);
        }

        // Reopen read cache. Should drop cache if it's still open
        let cache = $crate::loaders::homebrew::DATA_CACHE.$field.read().unwrap();
        let $classdata = cache.get($name);
        $($body)*
        drop(cache);
    };
}

// #[macro_export]
// macro_rules! read_homebrew_class {
//     ([$name:expr, $classdata:ident, $wrote:ident] => $($body:tt)*) => {
//         $crate::read_homebrew! { ClassData, classes [$name, $racedata, $wrote] =>
//             $($body)*
//         }
//     };
//     ([$name:expr, $classdata:ident] => $($body:tt)*) => {
//         read_homebrew_class! { [$name, $classdata, _a] =>
//             $($body)*
//         }
//     };
// }

// #[macro_export]
// macro_rules! read_homebrew_race {
//     ([$name:expr, $racedata:ident, $wrote:ident] => $($body:tt)*) => {
//         $crate::read_homebrew! { RaceData, races [$name, $racedata, $wrote] =>
//             $($body)*
//         }
//     };
//     ([$name:expr, $racedata:ident] => $($body:tt)*) => {
//         read_homebrew_race! { [$name, $racedata, _a] =>
//             $($body)*
//         }
//     };
// }
