use prost::Message;
use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;

use classes::*;
use races::*;

mod classes;
mod races;

fn write_proto(name: &str, data: &impl Message) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(name);

    let mut buf = vec![];
    _ = data.encode(&mut buf);

    fs::write(dest_path, &buf).unwrap();
}

fn main() -> Result<()> {
    generate_classes();
    generate_races();

    tauri_build::build();

    Ok(())
}
