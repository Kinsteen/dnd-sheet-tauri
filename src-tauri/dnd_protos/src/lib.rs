use std::collections::HashMap;

use protos::{character_sheet::HealthSystem, *};

pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));

    pub mod classes {
        include!(concat!(env!("OUT_DIR"), "/proto.classes.rs"));
    }
}

impl CharacterSheet {
    pub fn set_health_roll(&mut self) {
        self.health_system = Some(character_sheet::HealthSystem::Rolls(HealthRolls {
            rolls: vec![],
        }));
    }

    pub fn set_health_mean(&mut self) {
        self.health_system = Some(character_sheet::HealthSystem::Mean(true));
    }

    pub fn is_rolls(&mut self) -> Option<&mut HealthRolls> {
        let health_system = self.health_system.as_mut()?;

        match health_system {
            character_sheet::HealthSystem::Rolls(health_rolls) => {
                Some(health_rolls)
            }
            _ => {
                None
            }
        }
    }

    pub fn add_health_roll(&mut self, roll: i32) {
        if let Some(health_rolls) = self.is_rolls() {
            health_rolls.rolls.push(roll);
        }
    }
}

#[derive(Default, Clone)]
pub struct CharacterSheetBuilder {
    character_name: Option<String>,
    classes: Vec<Class>,
    race: Option<Race>,
    temp_health: Option<i32>,
    // TODO health system
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

    pub fn skill(mut self, skill: impl Into<String>) -> Self {
        self.skills.push(skill.into());
        self
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

        if self.race.is_none() {
            return Err("You have to pick a race.".to_string());
        }

        if self.health_system.is_none() {
            return Err("You have to pick a health system.".to_string());
        }

        Ok(())
    }

    pub fn build(self) -> Result<CharacterSheet, String> {
        self.can_build()?;

        Ok(CharacterSheet {
            character_name: self.character_name.unwrap(),
            classes: self.classes,
            race: self.race,
            health: 0,         // TODO get max health?
            temp_health: None, // TODO
            abilities: self.abilities,
            custom_ability_increases: self.custom_ability_increases,
            skills: self.skills,
            custom_languages: self.custom_languages,
            counters: self.counters,
            health_system: self.health_system,
        })
    }
}
