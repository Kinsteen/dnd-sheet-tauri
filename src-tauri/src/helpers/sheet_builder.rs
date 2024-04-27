use std::collections::HashMap;

use dnd_protos::protos::character_sheet::*;
use dnd_protos::protos::*;

use crate::{calculators::health::get_max_health, read_class, read_race};

#[derive(Default, Clone)]
pub struct CharacterSheetBuilder {
    character_name: Option<String>,
    classes: Vec<Class>,
    race: Option<Race>,
    temp_health: Option<i32>,
    health_system: Option<HealthSystem>,
    abilities: Vec<Ability>,
    custom_ability_increases: HashMap<String, i32>,
    skills: Vec<String>,
    custom_languages: Vec<String>,
    counters: Vec<Counter>,
}

impl CharacterSheetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        _ = self.character_name.insert(name.into());
        self
    }

    pub fn class(mut self, class: Class) -> Self {
        self.classes.push(class);
        self
    }

    pub fn race(mut self, race: Race) -> Self {
        _ = self.race.insert(race);
        self
    }

    pub fn temp_health(mut self, temp_health: i32) -> Self {
        _ = self.temp_health.insert(temp_health);
        self
    }

    pub fn ability(mut self, name: impl Into<String>, base_value: i32) -> Self {
        self.abilities.push(Ability {
            name: name.into(),
            base_value,
        });
        self
    }

    pub fn custom_ability_increase(mut self, ability: impl Into<String>, inc: i32) -> Self {
        self.custom_ability_increases.insert(ability.into(), inc);
        self
    }

    // TODO check if source and skill works
    pub fn skill(mut self, skill: impl Into<String>) -> Self {
        self.skills.push(skill.into());
        self
    }

    pub fn skill_source(mut self, source: impl Into<String>, skill: impl Into<String>) -> Self{
        todo!()
    }

    pub fn custom_language(mut self, language: impl Into<String>) -> Self {
        self.custom_languages.push(language.into());
        self
    }

    pub fn counter(mut self, name: impl Into<String>, used: i32) -> Self {
        self.counters.push(Counter {
            name: name.into(),
            used,
        });
        self
    }

    pub fn health_system(mut self, system: HealthSystem) -> Self {
        _ = self.health_system.insert(system);
        self
    }

    pub fn can_build(&self) -> Result<(), String> {
        if self.character_name.is_none() {
            return Err("Character name can't be empty.".to_string());
        };

        if self.classes.is_empty() {
            return Err("You have to pick a class.".to_string());
        }

        for class in self.classes.iter() {
            read_class!([&class.name, class_data] => {
                if class_data.is_none() {
                    return Err(format!("Class '{}' is not found in data!", &class.name));
                }
            });
        }

        if self.race.is_none() {
            return Err("You have to pick a race.".to_string());
        } else {
            read_race!([&self.race.as_ref().unwrap().name, race_data] => {
                if race_data.is_none() {
                    return Err(format!("Race '{}' is not found in data!", &self.race.as_ref().unwrap().name));
                }
            });
        }

        if self.health_system.is_none() {
            return Err("You have to pick a health system.".to_string());
        }

        Ok(())
    }

    pub fn build(self) -> Result<CharacterSheet, String> {
        self.can_build()?;
        
        let mut abilities_sorted = vec![];

        // If we have every abilities, we should sort them in the "normal" order
        if self.abilities.len() == 6 {
            let sorted_names = [
                "strength",
                "dexterity",
                "constitution",
                "intelligence",
                "wisdom",
                "charisma",
            ];

            for name in sorted_names {
                let a = self.abilities.iter().find(|a| a.name.eq(name)).unwrap();
                abilities_sorted.push(a.to_owned());
            }
        } else {
            abilities_sorted = self.abilities;
        }

        let mut sheet = CharacterSheet {
            character_name: self.character_name.unwrap(),
            classes: self.classes,
            race: self.race,
            health: 0,
            temp_health: None, // TODO?
            abilities: abilities_sorted,
            custom_ability_increases: self.custom_ability_increases,
            skills: self.skills,
            custom_languages: self.custom_languages,
            counters: self.counters,
            health_system: self.health_system,
        };

        sheet.health = get_max_health(&sheet);

        // Setup counters
        for class in sheet.classes.iter() {
            read_class!([&class.name, class_data] => {
                for counter in class_data.unwrap().counters.iter() {
                    sheet.counters.push(Counter {
                        name: counter.name.clone(),
                        used: 0,
                    });
                }
            });
        }

        Ok(sheet)
    }
}
