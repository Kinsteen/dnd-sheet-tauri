use crate::protos::*;

impl ClassData {
    pub fn get_cantrips_known(&self, level: i32) -> i32 {
        if level <= 0 || level > 30 {
            // 30 to be nice, DND5e is 20 max.
            return 0;
        }

        let mut temp_level = level;

        while !self.num_cantrips_known.contains_key(&temp_level) {
            temp_level -= 1;
        }

        *self.num_cantrips_known.get(&temp_level).unwrap()
    }
}

pub fn str_vec_to_string_vec(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}
