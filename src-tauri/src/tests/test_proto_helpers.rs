#[cfg(test)]
#[test]
fn cantrips_known_test() {
    use crate::helpers::utils::str_vec_to_string_vec;
    use dnd_protos::protos::*;
    use std::collections::HashMap;
    
    use crate::helpers::classdata::ClassDataHelper;

    let cleric = ClassData {
        name: "test".to_string(),
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
            num_spell_to_prepare: Some(spell_casting_data::NumSpellToPrepare::Formula("1".to_string())),
        }),
        custom_property: None,
        counters: vec![],
    };

    assert_eq!(cleric.get_cantrips_known(1), 3);
    assert_eq!(cleric.get_cantrips_known(2), 3);
    assert_eq!(cleric.get_cantrips_known(3), 3);
    assert_eq!(cleric.get_cantrips_known(4), 4);
    assert_eq!(cleric.get_cantrips_known(5), 4);
    assert_eq!(cleric.get_cantrips_known(6), 4);
    assert_eq!(cleric.get_cantrips_known(7), 4);
    assert_eq!(cleric.get_cantrips_known(8), 4);
    assert_eq!(cleric.get_cantrips_known(9), 4);
    assert_eq!(cleric.get_cantrips_known(10), 5);
    assert_eq!(cleric.get_cantrips_known(11), 5);
    assert_eq!(cleric.get_cantrips_known(12), 5);
    assert_eq!(cleric.get_cantrips_known(13), 5);
    assert_eq!(cleric.get_cantrips_known(14), 5);
    assert_eq!(cleric.get_cantrips_known(15), 5);
    assert_eq!(cleric.get_cantrips_known(20), 5);

    assert_eq!(cleric.get_cantrips_known(0), 0);
    assert_eq!(cleric.get_cantrips_known(-20), 0);
    assert_eq!(cleric.get_cantrips_known(25), 5);
    assert_eq!(cleric.get_cantrips_known(50), 0);
}
