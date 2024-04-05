use std::{collections::HashMap, sync::RwLock};

use dnd_protos::protos::{ClassData, RaceData, SkillData};
use once_cell::sync::Lazy;

// Field name should match field name in Homebrew struct
// Field type should match field type in Homebrew struct
pub struct Caches {
    pub classes: RwLock<HashMap<String, ClassData>>,
    pub races: RwLock<HashMap<String, RaceData>>,
    pub skills: RwLock<HashMap<String, SkillData>>,
}

pub static TEST_STRUCT: Lazy<Caches> = Lazy::new(|| Caches {
    classes: RwLock::new(HashMap::new()),
    races: RwLock::new(HashMap::new()),
    skills: RwLock::new(HashMap::new()),
});

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
        let cache = $crate::loaders::homebrew::TEST_STRUCT.$field.read().unwrap();
        if !cache.contains_key($name) {
            // We can arrive here with multiple threads if we're unlucky. Should not be a problem,
            // but may do unnecessary fs read.

            // Drop read cache, reopen write cache, write, then close
            drop(cache);

            // Load class from filesystem. If not found, should set $classdata to None and call
            // the body
            use std::fs;
            use prost::Message;
            // TODO unwrap properly
            let paths = fs::read_dir($crate::APP_STATE.read().unwrap()
                    .as_ref().unwrap()
                    .user_data.app_paths.homebrew_path.as_path()
                ).unwrap();
            $wrote = true;
            for path in paths {
                let real_path = path.unwrap().path();
                let data = fs::read(real_path).unwrap();
                let homebrew = dnd_protos::protos::Homebrew::decode(data.as_ref()).unwrap();
                for class in homebrew.$field {
                    let mut cache = $crate::loaders::homebrew::TEST_STRUCT.$field.write().unwrap();
                    cache.insert(class.name.clone(), class);
                    //drop(cache); // This drop is not important?
                }
            }
        } else {
            drop(cache);
        }

        // Reopen read cache. Should drop cache if it's still open
        let cache = $crate::loaders::homebrew::TEST_STRUCT.$field.read().unwrap();
        let $classdata = cache.get($name);
        $($body)*
        drop(cache);
    };
}

#[macro_export]
macro_rules! read_homebrew_class {
    ([$name:expr, $classdata:ident, $wrote:ident] => $($body:tt)*) => {
        $crate::read_homebrew! { ClassData, classes [$name, $racedata, $wrote] =>
            $($body)*
        }
    };
    ([$name:expr, $classdata:ident] => $($body:tt)*) => {
        read_homebrew_class! { [$name, $classdata, _a] =>
            $($body)*
        }
    };
}

#[macro_export]
macro_rules! read_homebrew_race {
    ([$name:expr, $racedata:ident, $wrote:ident] => $($body:tt)*) => {
        $crate::read_homebrew! { RaceData, races [$name, $racedata, $wrote] =>
            $($body)*
        }
    };
    ([$name:expr, $racedata:ident] => $($body:tt)*) => {
        read_homebrew_race! { [$name, $racedata, _a] =>
            $($body)*
        }
    };
}
