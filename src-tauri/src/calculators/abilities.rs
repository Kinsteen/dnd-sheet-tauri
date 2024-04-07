use dnd_protos::protos::*;

use crate::read_race;

pub fn calculate(name: &str, sheet: &CharacterSheet) -> Option<i32> {
    let v = &sheet.abilities;
    let ability = v.iter().find(|x| x.name.eq(name))?;
    let mut total = ability.base_value;

    if let Some(race) = &sheet.race {
        read_race!([&race.name, race_data] => {
            if race_data.is_some() {
                if let Some(inc) = race_data.unwrap().ability_increases.get(name) {
                    total += inc;
                }
            }
        });
    }

    if let Some(custom_inc) = sheet.custom_ability_increases.get(name) {
        total += custom_inc;
    }

    Some(total)
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
