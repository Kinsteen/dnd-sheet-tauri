use prost::Message;
use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;

use classes::*;
use races::*;
use skills::*;

mod classes;
mod races;
mod skills;

fn write_proto(name: &str, data: &impl Message) {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&root_dir).join("generated").join(name);
    _ = fs::create_dir_all(dest_path.parent().unwrap());
    let mut buf = vec![];
    _ = data.encode(&mut buf);

    fs::write(dest_path, &buf).unwrap();
}

fn main() -> Result<()> {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&root_dir).join("generated");
    _ = fs::remove_dir_all(dest_path);

    generate_classes();
    generate_races();
    generate_skills();

    tauri_build::build();

    Ok(())
}
