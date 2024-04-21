use std::fs;

use dnd_protos::protos::*;

use crate::{
    calculators::{
        abilities::{calculate, calculate_modifier, calculate_modifier_string, format_modifier},
        classes::{get_proficiency_bonus, get_total_level},
        health::get_max_health,
        utils::parse_expression,
    }, helpers::sheet_builder::CharacterSheetBuilder, list_skills, load_sheet_from_path, loaders::{
        homebrew::{load_in_cache, DATA_CACHE},
        r#static::get_full_class_name,
    }, read_class, read_race, ui_data::{AbilitiesDataUI, BasicDataClassUI, BasicDataUI, ClassUi, CounterUI, HealthUI, SkillDataUI, SkillsUI}, GeneratedAsset, APP_STATE
};

#[tauri::command]
pub fn get_abilities_data() -> Vec<AbilitiesDataUI> {
    let state = crate::APP_STATE.read();
    if state.is_none() {
        println!("state is none");
        return vec![];
    }

    let unwrapped_state = state.as_ref().unwrap();
    if unwrapped_state.user_data.sheet.is_none() {
        println!("no sheet");
        return vec![];
    }

    let sheet = unwrapped_state.user_data.sheet.as_ref().unwrap();

    let mut vec = vec![];
    for ability in &sheet.abilities {
        let modifier = calculate_modifier_string(&ability.name, sheet).unwrap();
        let total = format!("{}", calculate(&ability.name, sheet).unwrap());

        let mut proficient = false;
        let full_class_name = get_full_class_name(sheet.classes.first().unwrap());
        read_class!([full_class_name.as_str(), class_data] => {
            if class_data.is_some() {
                if class_data.unwrap().saving_throws.contains(&ability.name) {
                    proficient = true;
                }
            } else {
                eprintln!("Didn't find {}", full_class_name);
            }
        });

        vec.push(AbilitiesDataUI {
            name: ability.name.clone(),
            modifier,
            total,
            saving_throw: proficient,
            saving_throw_modifier: "+2".to_string(),
        })
    }

    drop(state);

    vec
}

#[tauri::command]
pub fn get_skills_data() -> Vec<SkillDataUI> {
    let state = crate::APP_STATE.read();
    if state.is_none() {
        return vec![];
    }

    let unwrapped_state = state.as_ref().unwrap();
    if unwrapped_state.user_data.sheet.is_none() {
        return vec![];
    }

    let sheet = unwrapped_state.user_data.sheet.as_ref().unwrap();

    let mut vec = vec![];

    load_in_cache(); // Load homebrews
    for skill_data in list_skills!() {
        let mut modifier = calculate_modifier(&skill_data.ability, sheet).unwrap_or(0);

        if sheet.skills.contains(&skill_data.name) {
            modifier += get_proficiency_bonus(sheet);
        }

        vec.push(SkillDataUI {
            name: skill_data.name.clone(),
            modifier: format_modifier(modifier),
            proficient: sheet.skills.contains(&skill_data.name),
        })
    }

    vec
}

#[tauri::command]
pub async fn get_counters() -> Result<Vec<CounterUI>, ()> {
    // let webview_window = tauri::WebviewWindowBuilder::new(
    //     &app,
    //     "label",
    //     tauri::WebviewUrl::App("index.html".into()),
    // )
    // .build()
    // .unwrap();
    // webview_window.set_always_on_top(true);

    use crate::calculators::utils::sparse_map_get;

    let mut state = crate::APP_STATE.write();
    if state.is_none() {
        return Ok(vec![]);
    }

    let unwrapped_state = state.as_mut().unwrap();
    if unwrapped_state.user_data.sheet.is_none() {
        return Ok(vec![]);
    }

    let sheet = unwrapped_state.user_data.sheet.as_mut().unwrap();

    let mut vec = vec![];

    let classes = &sheet.classes;

    for class in classes {
        let full_class_name = get_full_class_name(class);
        read_class!([full_class_name.as_str(), class_data] => {
            if class_data.is_some() {
                for counter in &class_data.unwrap().counters {
                    let mut max_uses = 0;
                    if let Some(stuff) = sparse_map_get(class.level, &counter.max_uses) {
                        max_uses = parse_expression(stuff, sheet).expect("Parsing didn't go well!");
                    }
                    // TODO should this be here?
                    if !sheet.counters.iter().any(|c| c.name.eq(&counter.name)) {
                        sheet.counters.push(Counter { name: counter.name.clone(), used: 0 });
                    }
                    vec.push(CounterUI {
                        name: counter.name.clone(),
                        used: sheet.counters.iter().find(|c| c.name.eq(&counter.name)).unwrap().used,
                        max_uses: max_uses as i32,
                    });
                }
            } else {
                eprintln!("Didn't find {}", full_class_name);
            }
        });
    }

    Ok(vec)
}

#[tauri::command]
pub async fn get_health() -> Result<HealthUI, ()> {
    let state = APP_STATE.read();
    let state = state.as_ref().unwrap();
    let Some(sheet) = &state.user_data.sheet else {
        return Err(());
    };

    Ok(HealthUI {
        current: sheet.health,
        max: get_max_health(sheet),
        temporary: 0,
    })
}

#[tauri::command]
pub async fn get_available_classes() -> Result<Vec<ClassUi>, ()> {
    load_in_cache(); // Load homebrews
    let mut vec = vec![];

    for path in GeneratedAsset::iter() {
        if let Some(name) = path.strip_prefix("classes/") {
            vec.push(ClassUi {
                name: name.to_string(),
            });
        }
    }

    let classes_cache = DATA_CACHE.classes.read();
    for (name, _data) in classes_cache.iter() {
        vec.push(ClassUi {
            name: name.to_string(),
        });
    }
    drop(classes_cache);

    vec.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(vec)
}

#[tauri::command]
pub async fn get_available_races() -> Result<Vec<ClassUi>, ()> {
    load_in_cache(); // Load homebrews
    let mut vec = vec![];

    for path in GeneratedAsset::iter() {
        if let Some(name) = path.strip_prefix("races/") {
            vec.push(ClassUi {
                name: name.to_string(),
            });
        }
    }

    let races_cache = DATA_CACHE.races.read();
    for (name, _data) in races_cache.iter() {
        vec.push(ClassUi {
            name: name.to_string(),
        });
    }
    drop(races_cache);

    vec.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(vec)
}

#[tauri::command]
pub async fn get_available_skills(
    class_name: String,
    race_name: String,
) -> Result<(SkillsUI, SkillsUI, SkillsUI), String> {
    let mut skills_class = vec![];
    let mut skills_race = vec![];
    let mut pick_class = 0;
    let mut pick_race = 0;

    // TODO background skills

    read_class!([&class_name, class_data] => {
        if let Some(class) = class_data {
            for skill in class.skill_proficiencies.clone() {
                skills_class.push(skill);
            }
            pick_class = class.num_skills_to_choose;
        }
    });

    read_race!([&race_name, race_data] => {
        if let Some(race) = race_data {
            for skill in race.skill_proficiencies.clone() {
                skills_race.push(skill);
            }
            pick_race = race.num_skills_to_choose;
        }
    });

    load_in_cache(); // Load homebrews
    Ok((
        SkillsUI {
            num_to_pick: pick_class,
            skills: skills_class,
        },
        SkillsUI {
            num_to_pick: pick_race,
            skills: skills_race,
        },
        SkillsUI {
            num_to_pick: 0,
            skills: list_skills!().iter().map(|s| s.name.clone()).collect(),
        },
    ))
}

#[tauri::command]
pub async fn calculate_ability(
    name: String,
    base_value: i32,
    class_name: String,
    race_name: String,
) -> Result<AbilitiesDataUI, String> {
    let mut builder = CharacterSheetBuilder::new();
    builder = builder.name("Bogus");
    builder = builder.ability(&name, base_value);
    builder = builder.class(Class {
        name: class_name,
        subclass: "".to_string(),
        level: 1,
        spell_slots: vec![],
        chosen_skills: vec![],
        used_cantrips: 0,
    });
    builder = builder.health_system(character_sheet::HealthSystem::Mean(true));
    builder = builder.race(Race { name: race_name });
    let sheet = builder.build().unwrap();
    Ok(AbilitiesDataUI {
        name: name.clone(),
        modifier: calculate_modifier_string(&name, &sheet).unwrap(),
        total: calculate(&name, &sheet).unwrap().to_string(),
        saving_throw: false,
        saving_throw_modifier: "".to_string(),
    })
}


#[tauri::command]
pub async fn get_basic_data() -> Result<BasicDataUI, String> {
    let state = crate::APP_STATE.read();
    let sheet = state.as_ref().unwrap().user_data.sheet.as_ref().unwrap();

    Ok(BasicDataUI {
        character_name: sheet.character_name.clone(),
        classes: sheet
            .classes
            .iter()
            .map(|c| BasicDataClassUI {
                name: c.name.clone(),
                level: c.level,
            })
            .collect(),
        race: sheet.race.as_ref().unwrap().name.clone(),
        total_level: get_total_level(sheet),
    })
}

#[tauri::command]
pub async fn get_sheets() -> Result<Vec<BasicDataUI>, String> {
    let state = crate::APP_STATE.read();
    let sheets_path = &state.as_ref().unwrap().user_data.app_paths.sheet_path;
    let paths = fs::read_dir(sheets_path).unwrap();

    let mut vec = vec![];

    for path in paths {
        // println!("Name: {}", path.unwrap().path().file_name().unwrap().to_string_lossy());
        let sheet = load_sheet_from_path(path.as_ref().unwrap().path().as_path());
        if sheet.is_ok() {
            let sheet = sheet.unwrap();
            vec.push(BasicDataUI {
                character_name: sheet.character_name.clone(),
                classes: sheet
                    .classes
                    .iter()
                    .map(|c| BasicDataClassUI {
                        name: c.name.clone(),
                        level: c.level,
                    })
                    .collect(),
                race: sheet.race.as_ref().unwrap().name.clone(),
                total_level: get_total_level(&sheet),
            })
        }
    }

    Ok(vec)
}
