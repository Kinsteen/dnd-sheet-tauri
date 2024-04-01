use std::collections::HashMap;

use dnd_protos::protos::*;
use eval::eval;

use super::{abilities::calculate_modifier, classes::get_total_level};

// Tested with get_cantrips_known
pub fn sparse_map_get<T>(index: i32, map: &HashMap<i32, T>) -> Option<&T> {
    let mut temp_index = index;

    while !map.contains_key(&temp_index) && temp_index >= 0 {
        temp_index -= 1;
    }

    map.get(&temp_index)
}

// TODO test this function
pub fn parse_expression(expression: &str, sheet: &CharacterSheet) -> Result<i64, eval::Error> {
    let mut final_expr: String = String::from(expression);

    final_expr = final_expr.replace(
        "str_mod",
        calculate_modifier("strength", sheet)
            .unwrap_or(0)
            .to_string()
            .as_str(),
    );
    final_expr = final_expr.replace(
        "dex_mod",
        calculate_modifier("dexterity", sheet)
            .unwrap_or(0)
            .to_string()
            .as_str(),
    );
    final_expr = final_expr.replace(
        "con_mod",
        calculate_modifier("constitution", sheet)
            .unwrap_or(0)
            .to_string()
            .as_str(),
    );
    final_expr = final_expr.replace(
        "int_mod",
        calculate_modifier("intelligence", sheet)
            .unwrap_or(0)
            .to_string()
            .as_str(),
    );
    final_expr = final_expr.replace(
        "wis_mod",
        calculate_modifier("wisdom", sheet)
            .unwrap_or(0)
            .to_string()
            .as_str(),
    );
    final_expr = final_expr.replace(
        "cha_mod",
        calculate_modifier("charisma", sheet)
            .unwrap_or(0)
            .to_string()
            .as_str(),
    );

    final_expr = final_expr.replace("total_level", get_total_level(sheet).to_string().as_str());

    for class in &sheet.classes {
        let class_name = class.name.clone();
        let to_replace = format!("{}_level", class_name);
        final_expr = final_expr.replace(&to_replace, class.level.to_string().as_str());
    }

    if let eval::Value::Number(result) = eval(&final_expr)? {
        Ok(result.as_i64().unwrap())
    } else {
        Err(eval::Error::Custom("Couldn't replace values".to_string()))
    }
}
