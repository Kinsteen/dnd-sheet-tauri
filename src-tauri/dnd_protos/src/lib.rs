use protos::*;

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
            character_sheet::HealthSystem::Rolls(health_rolls) => Some(health_rolls),
            _ => None,
        }
    }

    pub fn add_health_roll(&mut self, roll: i32) {
        if let Some(health_rolls) = self.is_rolls() {
            health_rolls.rolls.push(roll);
        }
    }
}
