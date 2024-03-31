use std::collections::HashMap;

use dnd_protos::proto_helpers::*;
use dnd_protos::protos::classes::*;
use dnd_protos::protos::*;

use crate::write_proto;

pub fn generate_classes() {
    // println!("cargo:warning=MESSAGE");
    let generic_cleric = ClassData {
        name: "cleric".to_string(),
        hit_die: 8,
        armor_proficiencies: str_vec_to_string_vec(vec!["light_armor"]),
        weapon_proficiencies: str_vec_to_string_vec(vec!["simple_weapon"]),
        tool_proficiencies: vec![],
        saving_throws: str_vec_to_string_vec(vec!["wisdom", "charisma"]),
        skill_proficiencies: str_vec_to_string_vec(vec![
            "history",
            "insight",
            "medicine",
            "persuasion",
            "religion",
        ]),
        num_skills_to_choose: 2,
        num_cantrips_known: HashMap::from([(1, 3), (4, 4), (10, 5)]),
        spell_slots: HashMap::from([]),
        num_spell_to_prepare: 3, // TODO
        custom_property: Some(class_data::CustomProperty::Cleric(Cleric {
            casting_ability: "wisdom".to_string(),
            subclass: None,
        })),
    };

    let mut light_cleric = generic_cleric.clone();
    if let Some(class_data::CustomProperty::Cleric(t)) = &mut light_cleric.custom_property {
        t.subclass = Some(classes::cleric::Subclass::Light(ClericLight {}));
    }

    write_proto("light_cleric", &light_cleric);
    println!("cargo::rerun-if-changed=build/classes.rs");
}
