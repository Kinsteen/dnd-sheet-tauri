use dnd_protos::protos::ClassData;

use crate::calculators::utils::sparse_map_get;

pub trait ClassDataHelper {
    fn get_cantrips_known(&self, level: i32) -> i32;
}

impl ClassDataHelper for ClassData {
    fn get_cantrips_known(&self, level: i32) -> i32 {
        if self.spellcasting.is_none() {
            return 0;
        }

        if level <= 0 || level > 30 {
            // 30 to be nice, DND5e is 20 max.
            return 0;
        }

        if let Some(i) = sparse_map_get(
            level,
            &self.spellcasting.clone().unwrap().num_cantrips_known,
        ) {
            *i
        } else {
            0
        }
    }
}
