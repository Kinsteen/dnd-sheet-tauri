pub fn str_vec_to_string_vec(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}
