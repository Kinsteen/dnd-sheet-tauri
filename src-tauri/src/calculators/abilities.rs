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

pub fn calculate_bonus(name: &str, sheet: &CharacterSheet) -> Option<String> {
    let total = calculate(name, sheet);

    if let Some(total_int) = total {
        let modifier = (total_int - 10) / 2;
        println!("{}", total_int - 10);
        println!("{}", modifier);
        if modifier >= 0 {
            Some(format!("+{}", modifier))
        } else {
            Some(format!("{}", modifier))
        }
    } else {
        None
    }
}
