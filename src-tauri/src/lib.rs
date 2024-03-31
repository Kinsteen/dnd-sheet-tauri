use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/generated"]
pub struct Asset;

pub mod calculators {
    pub mod abilities;
}

pub mod loaders {
    pub mod r#static;
}

mod tests {
    mod test_proto_helpers;
    mod test_calculators;
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
