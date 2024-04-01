use std::collections::HashMap;

use dnd_protos::protos::classes::*;
use dnd_protos::protos::*;

use crate::str_vec_to_string_vec;
use crate::write_proto;

pub fn generate_classes() {
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
        spellcasting: Some(SpellCastingData {
            casting_ability: "wisdom".to_string(),
            num_cantrips_known: HashMap::from([(1, 3), (4, 4), (10, 5)]),
            spell_slots: HashMap::from([]),
            num_spell_to_prepare: Some(spell_casting_data::NumSpellToPrepare::Formula(
                "wis_mod + cleric_level".to_string(),
            )),
        }),
        counters: vec![
            CounterData {
                name: "channel_divinity".to_string(),
                max_uses: HashMap::from([
                    (1, "1".to_string()),
                    (6, "2".to_string()),
                    (18, "3".to_string()),
                ]),
                resets_on_short_rest: true,
                resets_on_long_rest: true,
                custom_reset: String::new(),
            },
            CounterData {
                name: "harness_divine_power".to_string(),
                max_uses: HashMap::from([
                    (2, "1".to_string()),
                    (6, "2".to_string()),
                    (18, "3".to_string()),
                ]),
                resets_on_short_rest: false,
                resets_on_long_rest: true,
                custom_reset: String::new(),
            },
        ],
        custom_property: Some(class_data::CustomProperty::Cleric(Cleric {
            casting_ability: "wisdom".to_string(),
            subclass: None,
        })),
    };

    let mut light_cleric = generic_cleric.clone();
    let warding_flare = CounterData {
        name: "warding_flare".to_string(),
        max_uses: HashMap::from([(1, "wis_mod".to_string())]),
        resets_on_short_rest: false,
        resets_on_long_rest: true,
        custom_reset: String::new(),
    };
    light_cleric.counters.push(warding_flare);
    if let Some(class_data::CustomProperty::Cleric(t)) = &mut light_cleric.custom_property {
        t.subclass = Some(classes::cleric::Subclass::Light(ClericLight {}));
    }
    write_proto("classes/light_cleric", &light_cleric);

    let mut life_cleric = generic_cleric.clone();
    life_cleric
        .armor_proficiencies
        .push("heavy_armor".to_string());
    write_proto("classes/life_cleric", &light_cleric);

    let barbarian = ClassData {
        name: "barbarian".to_string(),
        hit_die: 12,
        armor_proficiencies: str_vec_to_string_vec(vec!["light_armor", "medium_armor", "shields"]),
        weapon_proficiencies: str_vec_to_string_vec(vec!["simple_weapon", "martial_weapons"]),
        tool_proficiencies: vec![],
        saving_throws: str_vec_to_string_vec(vec!["strength", "constitution"]),
        skill_proficiencies: str_vec_to_string_vec(vec![
            "animal_handling",
            "athletics",
            "intimidation",
            "nature",
            "perception",
            "survival",
        ]),
        num_skills_to_choose: 2,
        spellcasting: None,
        counters: vec![CounterData {
            name: "rage".to_string(),
            max_uses: HashMap::from([
                (1, "2".to_string()),
                (3, "3".to_string()),
                (6, "4".to_string()),
                (12, "5".to_string()),
                (17, "6".to_string()),
                (20, "inf".to_string()), // TODO How will that work
            ]),
            resets_on_short_rest: false,
            resets_on_long_rest: true,
            custom_reset: String::new(),
        }],
        custom_property: None,
    };
    write_proto("classes/barbarian", &barbarian);

    let bard = ClassData {
        name: "bard".to_string(),
        hit_die: 8,
        armor_proficiencies: str_vec_to_string_vec(vec!["light_armor"]),
        weapon_proficiencies: str_vec_to_string_vec(vec![
            "simple_weapon",
            "hand_crossbows",
            "longswords",
            "rapiers",
            "shortswords",
        ]),
        tool_proficiencies: vec![], // TODO three musical instruments
        saving_throws: str_vec_to_string_vec(vec!["dexterity", "charisma"]),
        skill_proficiencies: str_vec_to_string_vec(vec!["any", "any", "any"]), // TODO any three
        num_skills_to_choose: 0,
        spellcasting: Some(SpellCastingData {
            casting_ability: "charisma".to_string(),
            num_cantrips_known: HashMap::from([(1, 2), (4, 3), (10, 4)]),
            spell_slots: HashMap::from([]),
            num_spell_to_prepare: Some(spell_casting_data::NumSpellToPrepare::LevelMap(
                dnd_protos::protos::LevelMap {
                    level_map: HashMap::from([
                        (1, 4),
                        (2, 5),
                        (3, 6),
                        (4, 7),
                        (5, 8),
                        (6, 9),
                        (7, 10),
                        (8, 11),
                        (9, 12),
                        (10, 14),
                        (11, 15),
                        (13, 16),
                        (14, 18),
                        (15, 19),
                        (17, 20),
                        (18, 22),
                    ]),
                },
            )),
        }),
        counters: vec![CounterData {
            name: "bardic_inspiration".to_string(),
            max_uses: HashMap::from([(1, "cha_mod".to_string())]),
            resets_on_short_rest: false,
            resets_on_long_rest: true,
            custom_reset: String::from("When lvl 5 or more, it also resets on short rest"),
        }],
        custom_property: None,
    };
    write_proto("classes/bard", &bard);

    let druid = ClassData {
        name: "druid".to_string(),
        hit_die: 8,
        armor_proficiencies: str_vec_to_string_vec(vec!["light_armor", "medium_armor", "shields"]),
        weapon_proficiencies: str_vec_to_string_vec(vec![
            "clubs",
            "daggers",
            "darts",
            "javelins",
            "maces",
            "quarterstaffs",
            "scimitars",
            "sickles",
            "slings",
            "spears",
        ]),
        tool_proficiencies: str_vec_to_string_vec(vec!["herbalism_kit"]),
        saving_throws: str_vec_to_string_vec(vec!["intelligence", "wisdom"]),
        skill_proficiencies: str_vec_to_string_vec(vec![
            "arcana",
            "animal_handling",
            "insight",
            "medicine",
            "nature",
            "perception",
            "religion",
            "survival",
        ]),
        num_skills_to_choose: 2,
        spellcasting: Some(SpellCastingData {
            casting_ability: "wisdom".to_string(),
            num_cantrips_known: HashMap::from([(1, 2), (4, 3), (10, 4)]),
            spell_slots: HashMap::from([]),
            num_spell_to_prepare: Some(spell_casting_data::NumSpellToPrepare::Formula(
                "wis_mod + druid_level".to_string(),
            )),
        }),
        counters: vec![CounterData {
            name: "wild_shape".to_string(),
            max_uses: HashMap::from([
                (2, "2".to_string()),
                (20, "inf".to_string()), // TODO inf
            ]),
            resets_on_short_rest: true,
            resets_on_long_rest: true,
            custom_reset: String::new(),
        }],
        custom_property: None,
    };
    write_proto("classes/druid", &druid);

    let fighter = ClassData {
        name: "fighter".to_string(),
        hit_die: 10,
        armor_proficiencies: str_vec_to_string_vec(vec![
            "light_armor",
            "medium_armor",
            "heavy_armor",
            "shields",
        ]),
        weapon_proficiencies: str_vec_to_string_vec(vec!["simple_weapon", "martial_weapons"]),
        tool_proficiencies: vec![],
        saving_throws: str_vec_to_string_vec(vec!["strength", "constitution"]),
        skill_proficiencies: str_vec_to_string_vec(vec![
            "acrobatics",
            "animal_handling",
            "athletics",
            "history",
            "insight",
            "intimidation",
            "perception",
            "survival",
        ]),
        num_skills_to_choose: 2,
        spellcasting: None,
        counters: vec![
            CounterData {
                name: "second_wind".to_string(),
                max_uses: HashMap::from([(1, "1".to_string())]),
                resets_on_short_rest: true,
                resets_on_long_rest: true,
                custom_reset: String::new(),
            },
            CounterData {
                name: "action_surge".to_string(),
                max_uses: HashMap::from([(1, "1".to_string()), (17, "2".to_string())]),
                resets_on_short_rest: true,
                resets_on_long_rest: true,
                custom_reset: String::new(),
            },
            CounterData {
                name: "indomitable".to_string(),
                max_uses: HashMap::from([
                    (9, "1".to_string()),
                    (13, "2".to_string()),
                    (17, "3".to_string()),
                ]),
                resets_on_short_rest: false,
                resets_on_long_rest: true,
                custom_reset: String::new(),
            },
        ],
        custom_property: None,
    };
    write_proto("classes/fighter", &fighter);

    println!("cargo::rerun-if-changed=build/classes.rs");
}
