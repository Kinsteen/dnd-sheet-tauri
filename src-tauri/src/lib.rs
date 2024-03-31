pub mod calculators {
    pub mod abilities;
}

mod tests {
    mod test_proto_helpers;
}

pub mod helpers {
    pub fn str_vec_to_string_vec(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }
}

pub mod ui_data {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct AbilitiesDataUI {
        pub name: &'static str,
        pub modifier: &'static str,
        pub total: &'static str,
        pub saving_throw: bool,
        pub saving_throw_modifier: &'static str,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SkillDataUI {
        pub name: &'static str,
        pub modifier: &'static str,
        pub proficient: bool,
    }
}
