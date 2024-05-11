use dnd_protos::protos::*;

use crate::str_vec_to_string_vec;
use crate::write_proto;

pub fn generate_backgrounds() {
    let acolyte = BackgroundData {
        name: "acolyte".to_string(),
        languages_known: str_vec_to_string_vec(vec!["any", "any"]),
        skill_proficiencies: str_vec_to_string_vec(vec!["insight", "religion"]),
        tool_proficiencies:str_vec_to_string_vec(vec![]),
    };

    write_proto("backgrounds/acolyte", &acolyte);
}
