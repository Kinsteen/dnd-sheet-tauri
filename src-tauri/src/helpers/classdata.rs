use dnd_protos::protos::ClassData;

pub trait ClassDataHelper {
    fn get_cantrips_known(&self, level: i32) -> i32;
}

impl ClassDataHelper for ClassData {
    fn get_cantrips_known(&self, level: i32) -> i32 {
        if level <= 0 || level > 30 {
            // 30 to be nice, DND5e is 20 max.
            return 0;
        }

        let mut temp_level = level;

        while !self
            .spellcasting
            .clone()
            .unwrap()
            .num_cantrips_known
            .contains_key(&temp_level)
        {
            temp_level -= 1;
        }

        *self
            .spellcasting
            .clone()
            .unwrap()
            .num_cantrips_known
            .get(&temp_level)
            .unwrap()
    }
}
