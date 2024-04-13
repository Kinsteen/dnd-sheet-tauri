use dnd_protos::protos::character_sheet::*;
use dnd_protos::protos::*;

// use crate::loaders::r#static::read_class;

use crate::loaders::r#static::get_full_class_name;
use crate::read_class;

use super::abilities::calculate_modifier;
use super::classes::get_total_level;

pub fn get_max_health(sheet: &CharacterSheet) -> i32 {
    if sheet.health_system.is_none() {
        eprintln!("No health system, can't calculate max hp");
        return 0;
    }

    let health_system_clone = sheet.health_system.clone();

    let hit_die;
    read_class!([get_full_class_name(sheet.classes.first().unwrap()).as_str(), class_data] => {
        if class_data.is_none() {
            return 0;
        }

        hit_die = class_data.unwrap().hit_die;
    });

    let con_mod = calculate_modifier("constitution", sheet).unwrap();
    let mut total_health = hit_die + con_mod;

    match health_system_clone.unwrap() {
        HealthSystem::Mean(_m) => {
            for (i, class) in sheet.classes.iter().enumerate() {
                let hit_die;
                read_class!([get_full_class_name(class).as_str(), class_data] => {
                    if class_data.is_none() {
                        return 0;
                    }
                    hit_die = class_data.unwrap().hit_die;
                });

                let hit_die_mean = ((hit_die as f32) / 2f32 + 1f32).ceil() as i32;

                // This is because level 1 is a bit different, because we have the full
                // hit die + con mod, and it's calculated before
                let level_modifier = if i == 0 { -1 } else { 0 };

                total_health += (hit_die_mean + con_mod) * (class.level + level_modifier);
            }
        }
        HealthSystem::Rolls(r) => {
            let rolls = r.rolls;

            if rolls.len() as i32 != (get_total_level(sheet) - 1) {
                eprintln!(
                    "Number of rolls does not match total level: {} != {}",
                    rolls.len(),
                    (get_total_level(sheet) - 1)
                );
                return 0;
            }

            for roll in rolls {
                total_health += roll + con_mod;
            }
        }
    }

    total_health
}
