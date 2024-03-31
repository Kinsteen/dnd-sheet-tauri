use dnd_protos::protos::*;

use crate::read_proto;

pub fn calculate(name: &str, sheet: &CharacterSheet) -> Option<i32> {
    let v = &sheet.abilities;
    if let Some(ability) = v.iter().find(|x| x.name.eq(name)) {
        let mut total = ability.base_value;

        if let Some(race) = &sheet.race {
            if let Some(race_data) = read_proto!(race.name, RaceData) {
                if let Some(inc) = race_data.ability_increases.get(name) {
                    total += inc;
                }
            }
        }

        if let Some(custom_inc) = sheet.custom_ability_increases.get(name) {
            total += custom_inc;
        }

        Some(total)
    } else {
        eprintln!("Can't find ability {}", name);
        None
    }
}

pub fn format_modifier(modifier: i32) -> String {
    if modifier >= 0 {
        format!("+{}", modifier)
    } else {
        format!("{}", modifier)
    }
}

pub fn calculate_modifier(name: &str, sheet: &CharacterSheet) -> Option<i32> {
    calculate(name, sheet).map(|total| (((total - 10) as f32) / 2f32).floor() as i32)
}

pub fn calculate_modifier_string(name: &str, sheet: &CharacterSheet) -> Option<String> {
    calculate_modifier(name, sheet).map(format_modifier)
}
