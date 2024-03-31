use std::collections::HashMap;

use dnd_protos::proto_helpers::*;
use dnd_protos::protos::*;

use crate::write_proto;

pub fn generate_races() {
    let goldwalker_ra = RaceData {
        name: "godwalker_ra".to_string(),
        ability_increases: HashMap::from([
            ("wisdom".to_string(), 2),
            ("intelligence".to_string(), 1),
        ]),
        walking_speed: 30,
        size: "medium".to_string(),
        languages_known: str_vec_to_string_vec(vec!["common", "ancient_egyptian"]),
        skill_proficiencies: str_vec_to_string_vec(vec!["perception"]),
        num_skills_to_choose: 1,
    };

    write_proto("godwalker_ra", &goldwalker_ra);
    println!("cargo::rerun-if-changed=build/races.rs");
}
