use std::collections::HashMap;

use crate::{str_vec_to_string_vec, write_test_proto};

use dnd_protos::protos::*;

pub fn generate_test_homebrew() {
    let goldwalker_ra = RaceData {
        name: "godwalker_ra".to_string(),
        ability_increases: HashMap::from([
            ("wisdom".to_string(), 2),
            ("constitution".to_string(), 1),
        ]),
        walking_speed: 30,
        size: "medium".to_string(),
        languages_known: str_vec_to_string_vec(vec!["common", "ancient_egyptian"]),
        skill_proficiencies: str_vec_to_string_vec(vec!["perception"]),
        num_skills_to_choose: 1,
        tool_proficiencies: vec![],
        num_tools_to_choose: 0,
        weapon_proficiencies: vec![],
        num_weapons_to_choose: 0,
        darkvision_range: None,
    };

    // https://www.dandwiki.com/wiki/A.I._(5e_Race)
    let ai = RaceData {
        name: "ai".to_string(),
        ability_increases: HashMap::from([
            ("intelligence".to_string(), 2),
            ("wisdom".to_string(), 1),
        ]),
        walking_speed: 30,
        size: "medium".to_string(),
        languages_known: str_vec_to_string_vec(vec!["common", "any"]),
        skill_proficiencies: str_vec_to_string_vec(vec!["investigation", "perception", "any"]),
        num_skills_to_choose: 0,
        tool_proficiencies: vec![],
        num_tools_to_choose: 0,
        weapon_proficiencies: vec![],
        num_weapons_to_choose: 0,
        darkvision_range: None,
    };

    let blood_hunter = ClassData {
        name: "blood_hunter".to_string(),
        hit_die: 10,
        ..Default::default()
    };

    let palg_homebrew = Homebrew {
        name: "palg".to_string(),
        classes: vec![blood_hunter],
        races: vec![goldwalker_ra, ai],
        skills: vec![],
    };
    write_test_proto("palg", &palg_homebrew);

    println!("cargo::rerun-if-changed=build/test_homebrew.rs");
}
