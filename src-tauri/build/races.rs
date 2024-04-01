use std::collections::HashMap;

use dnd_protos::proto_helpers::*;
use dnd_protos::protos::*;

use crate::write_proto;

pub fn generate_races() {
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
        darkvision_range: None
    };
    write_proto("races/godwalker_ra", &goldwalker_ra);

    // TODO subrace
    let dragonborn = RaceData {
        name: "dragonborn".to_string(),
        ability_increases: HashMap::from([
            ("strength".to_string(), 2),
            ("charisma".to_string(), 1)
        ]),
        walking_speed: 30,
        size: "medium".to_string(),
        languages_known: str_vec_to_string_vec(vec!["common", "draconic"]),
        skill_proficiencies: vec![],
        num_skills_to_choose: 0,
        tool_proficiencies: vec![],
        num_tools_to_choose: 0,
        weapon_proficiencies: vec![],
        num_weapons_to_choose: 0,
        darkvision_range: None
    };
    write_proto("races/dragonborn", &dragonborn);

    let dwarf = RaceData {
        name: "dwarf".to_string(),
        ability_increases: HashMap::from([
            ("constitution".to_string(), 2)
        ]),
        walking_speed: 25,
        size: "medium".to_string(),
        languages_known: str_vec_to_string_vec(vec!["common", "dwarvish"]),
        skill_proficiencies: vec![],
        num_skills_to_choose: 0,
        tool_proficiencies: str_vec_to_string_vec(vec!["smiths_tools", "brewers_supplies", "masons_tools"]),
        num_tools_to_choose: 1,
        weapon_proficiencies: str_vec_to_string_vec(vec!["battleaxe", "handaxe", "light_hammer", "warhammer"]),
        num_weapons_to_choose: 0,
        darkvision_range: Some(60)
    };
    write_proto("races/dwarf", &dwarf);

    let mut hill_dwarf = dwarf.clone();
    hill_dwarf.name = "hill_dwarf".to_string();
    hill_dwarf.ability_increases.insert("wisdom".to_string(), 1);
    write_proto("races/hill_dwarf", &hill_dwarf);

    let elf = RaceData {
        name: "elf".to_string(),
        ability_increases: HashMap::from([
            ("dexterity".to_string(), 2)
        ]),
        walking_speed: 30,
        size: "medium".to_string(),
        languages_known: str_vec_to_string_vec(vec!["common", "elvish"]),
        skill_proficiencies: str_vec_to_string_vec(vec!["perception"]),
        num_skills_to_choose: 0,
        tool_proficiencies: vec![],
        num_tools_to_choose: 0,
        weapon_proficiencies: vec![],
        num_weapons_to_choose: 0,
        darkvision_range: Some(60)
    };
    write_proto("races/elf", &elf);

    let mut high_elf = elf.clone();
    high_elf.name = "high_elf".to_string();
    high_elf.ability_increases.insert("intelligence".to_string(), 1);
    high_elf.weapon_proficiencies.push("longsword".to_string());
    high_elf.weapon_proficiencies.push("shortsword".to_string());
    high_elf.weapon_proficiencies.push("shortbow".to_string());
    high_elf.weapon_proficiencies.push("longbow".to_string());
    high_elf.languages_known.push("choose".to_string());
    write_proto("races/high_elf", &high_elf);

    let tiefling = RaceData {
        name: "tiefling".to_string(),
        ability_increases: HashMap::from([
            ("intelligence".to_string(), 1),
            ("charisma".to_string(), 2)
        ]),
        walking_speed: 30,
        size: "medium".to_string(),
        languages_known: str_vec_to_string_vec(vec!["common", "infernal"]),
        skill_proficiencies: vec![],
        num_skills_to_choose: 0,
        tool_proficiencies: vec![],
        num_tools_to_choose: 0,
        weapon_proficiencies: vec![],
        num_weapons_to_choose: 0,
        darkvision_range: Some(60)
    };
    write_proto("races/tiefling", &tiefling);

    println!("cargo::rerun-if-changed=build/races.rs");
}
