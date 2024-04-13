#[macro_export]
macro_rules! read_class {
    ([$name:expr, $class_data:ident, $wrote:ident] => $($body:tt)*) => {{
        // Check first if it's builtin
        use dnd_protos::protos::ClassData;
        let builtin = $crate::read_proto!(format!("classes/{}", $name), ClassData);
        if builtin.is_some() {
            let $wrote = false;
            let $class_data = builtin; // Move
            $($body)*
        } else {
            $crate::read_homebrew! { ClassData, classes [$name, $class_data, $wrote] =>
                $($body)*
            }
        }
    }};
    ([$name:expr, $class_data:ident] => $($body:tt)*) => {
        read_class! { [$name, $class_data, _trash_write] =>
            $($body)*
        }
    };
}

#[macro_export]
macro_rules! read_race {
    ([$name:expr, $race_data:ident, $wrote:ident] => $($body:tt)*) => {{
        // Check first if it's builtin
        use dnd_protos::protos::RaceData;
        let builtin = $crate::read_proto!(format!("races/{}", $name), RaceData);
        if builtin.is_some() {
            let $wrote = false;
            let $race_data = builtin; // Move
            $($body)*
        } else {
            $crate::read_homebrew! { RaceData, races [$name, $race_data, $wrote] =>
                $($body)*
            }
        }
    }};
    ([$name:expr, $race_data:ident] => $($body:tt)*) => {
        read_race! { [$name, $race_data, _trash_write] =>
            $($body)*
        }
    };
}

// TODO test
#[macro_export]
macro_rules! read_skill {
    ([$name:expr, $skill_data:ident, $wrote:ident] => $($body:tt)*) => {{
        // Check first if it's builtin
        use dnd_protos::protos::SkillData;
        let builtin = $crate::read_proto!(format!("skills/{}", $name), SkillData);
        if builtin.is_some() {
            let $wrote = false;
            let $skill_data = builtin; // Move
            $($body)*
        } else {
            $crate::read_homebrew! { SkillData, skills [$name, $skill_data, $wrote] =>
                $($body)*
            }
        }
    }};
    ([$name:expr, $skill_data:ident] => $($body:tt)*) => {
        read_skill! { [$name, $skill_data, _trash_write] =>
            $($body)*
        }
    };
}

// TODO test
#[macro_export]
macro_rules! read_background {
    ([$name:expr, $bg_data:ident, $wrote:ident] => $($body:tt)*) => {{
        // Check first if it's builtin
        use dnd_protos::protos::BackgroundData;
        let builtin = $crate::read_proto!(format!("backgrounds/{}", $name), BackgroundData);
        if builtin.is_some() {
            let $wrote = false;
            let $race_data = builtin; // Move
            $($body)*
        } else {
            $crate::read_homebrew! { BackgroundData, backgrounds [$name, $bg_data, $wrote] =>
                $($body)*
            }
        }
    }};
    ([$name:expr, $bg_data:ident] => $($body:tt)*) => {
        read_background! { [$name, $bg_data, _trash_write] =>
            $($body)*
        }
    };
}

// TODO test
#[macro_export]
macro_rules! list_skills {
    () => {{
        use $crate::read_proto;
        let mut skills = vec![];

        // Builtin first
        for path in GeneratedAsset::iter() {
            if path.starts_with("skills/") {
                if let Some(skill_data) = read_proto!(path, SkillData) {
                    skills.push(skill_data);
                }
            }
        }

        let cache = $crate::loaders::homebrew::DATA_CACHE.skills.read().unwrap();
        for (_name, data) in cache.iter() {
            skills.push(data.clone());
        }
        drop(cache);

        skills.sort_by(|a, b| a.name.cmp(&b.name));

        skills
    }};
}
