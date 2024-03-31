use dnd_protos::protos::*;

pub fn calculate(name: &str, sheet: &CharacterSheet) -> Option<i32> {
    let v = &sheet.abilities;
    if let Some(ability) = v.iter().find(|x| x.name.eq(name)) {
        println!("Ability name: {}", ability.name);
        println!("{}", ability.base_value);
        Some(12)
    } else {
        eprintln!("Can't find ability {}", name);
        None
    }
}
