use dnd_protos::protos::CharacterSheet;

pub fn get_total_level(sheet: &CharacterSheet) -> i32 {
    sheet.classes.iter().fold(0, |acc, c| acc + c.level)
}

pub fn get_proficiency_bonus(sheet: &CharacterSheet) -> i32 {
    let total_level = get_total_level(sheet);

    match total_level {
        1..=4 => 2,
        5..=8 => 3,
        9..=12 => 4,
        13..=16 => 5,
        17..=20 => 6,
        _ => 0
    }
}
