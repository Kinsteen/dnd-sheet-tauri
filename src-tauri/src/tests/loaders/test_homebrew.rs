#[cfg(test)]

struct TestNecessities {
    user_data: crate::UserData,
}

fn init() {
    use std::path::PathBuf;

    let mut state = crate::APP_STATE.write().unwrap();
    *state = Some(crate::State {
        user_data: crate::UserData {
            sheet: None,
            app_paths: crate::AppPaths {
                user_data_path: PathBuf::from("./test_data_saved"),
                sheet_path: PathBuf::from("./test_data"),
                homebrew_path: PathBuf::from("src/tests/resources/homebrew"),
            },
        },
    });
}

#[test]
fn homebrew_test() {
    use crate::{read_class, read_race};
    init();

    read_race!(["elf", race_data, write] => {
        assert!(race_data.is_some());
        assert!(!write);
    });

    read_race!(["godwalker_ra", race_data, write] => {
        assert!(race_data.is_some());
        let race_data = race_data.unwrap();
        assert_eq!(race_data.name, "godwalker_ra");
        assert_eq!(race_data.walking_speed, 30);
        assert_eq!(race_data.size, "medium");
        assert!(!write);
    });

    read_race!(["godwalker_ra", race_data, write] => {
        assert!(race_data.is_some());
        let race_data = race_data.unwrap();
        assert_eq!(race_data.name, "godwalker_ra");
        assert_eq!(race_data.walking_speed, 30);
        assert_eq!(race_data.size, "medium");
        assert!(!write);
    });

    read_race!(["non_existent", race_data, write] => {
        assert!(race_data.is_none());
        assert!(!write); // Each time we search for something that doesn't exist, we read the FS
    });

    read_race!(["ai", race_data, write] => {
        assert!(race_data.is_some());
        assert!(!write); // already written when loading godwalker
    });

    read_class!(["light_cleric", class_data, write] => {
        assert!(class_data.is_some());
        assert!(!write);
    });

    read_class!(["blood_hunter", class_data, write] => {
        assert!(class_data.is_some());
        assert!(!write);
    });

    read_class!(["non_existent", class_data, write] => {
        assert!(class_data.is_none());
        assert!(!write);
    });

    read_class!(["blood_hunter", class_data, write] => {
        assert!(class_data.is_some());
        assert!(!write);
    });

    // Concurrent tests, should not deadlock!
    // 1000 threads is so many more than IRL
    // We use non_existent to force write lock
    let mut threads = vec![];
    for _ in 0..1000 {
        threads.push(std::thread::spawn(|| {
            read_race! { ["godwalker_ra", race_data] =>
                assert!(race_data.is_some());
                let race_data = race_data.unwrap();
                assert_eq!(race_data.name, "godwalker_ra");
                assert_eq!(race_data.walking_speed, 30);
                assert_eq!(race_data.size, "medium");
            };

            read_race! { ["dragonborn", race_data] =>
                assert!(race_data.is_some());
            };

            read_race! { ["non_existent", race_data] =>
                assert!(race_data.is_none());
            };

            read_race! { ["ai", race_data] =>
                assert!(race_data.is_some());
            };

            read_class!(["non_existent", class_data] => {
                assert!(class_data.is_none());
            });

            read_class!(["blood_hunter", class_data] => {
                assert!(class_data.is_some());
            });

            read_class!(["non_existent", class_data] => {
                assert!(class_data.is_none());
            });

            read_class!(["blood_hunter", class_data] => {
                assert!(class_data.is_some());
            });
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
