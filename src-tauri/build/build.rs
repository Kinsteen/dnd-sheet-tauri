use prost::Message;
use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;

use backgrounds::*;
use classes::*;
use races::*;
use skills::*;
use test_homebrew::*;

mod backgrounds;
mod classes;
mod races;
mod skills;
mod test_homebrew;

pub fn str_vec_to_string_vec(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

fn write_proto(name: &str, data: &impl Message) {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&root_dir).join("generated").join(name);
    _ = fs::create_dir_all(dest_path.parent().unwrap());
    let mut buf = vec![];
    _ = data.encode(&mut buf);

    fs::write(dest_path, &buf).unwrap();
}

fn write_test_proto(name: &str, data: &impl Message) {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&root_dir).join("generated/tests").join(name);
    _ = fs::create_dir_all(dest_path.parent().unwrap());
    let mut buf = vec![];
    _ = data.encode(&mut buf);

    fs::write(dest_path, &buf).unwrap();
}

fn main() -> Result<()> {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&root_dir).join("generated");
    _ = fs::remove_dir_all(dest_path);
    let dest_path = Path::new(&root_dir).join("src/tests/resources");
    _ = fs::remove_dir_all(dest_path);

    generate_backgrounds();
    generate_classes();
    generate_races();
    generate_skills();

    generate_test_homebrew();

    tauri_build::build();

    Ok(())
}
