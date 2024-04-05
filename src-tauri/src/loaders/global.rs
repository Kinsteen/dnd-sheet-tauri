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

