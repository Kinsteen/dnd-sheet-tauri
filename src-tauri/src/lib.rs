use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/generated"]
pub struct GeneratedAsset;

pub mod calculators {
    pub mod abilities;
    pub mod classes;
    pub mod utils;
}

pub mod helpers {
    pub mod classdata;
    pub mod utils;
}

pub mod loaders {
    pub mod r#static;
}

mod tests {
    mod test_calculators;
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
