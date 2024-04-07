use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        //.type_attribute("ClericLight", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile(
            &[
                "src/protos/dnd_data.proto",
                "src/protos/character_sheet.proto",
                "src/protos/homebrew.proto",
                "src/protos/user_data.proto",
            ],
            &["src/protos"],
        )?;

    Ok(())
}
