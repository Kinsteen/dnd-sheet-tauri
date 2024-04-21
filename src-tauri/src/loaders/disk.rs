use std::{fs, path::Path};

use dnd_protos::protos::{CharacterSheet, UserData};
use prost::Message;

#[derive(Debug)]
pub enum DiskError {
    NoState,
    DecodeError,
    FileNotFound,
}

/// Reads from APP_STATE
pub fn load_current_user_data() -> Result<UserData, DiskError> {
    let state = crate::APP_STATE.read();
    if state.is_none() {
        return Err(DiskError::NoState);
    }

    let state = state.as_ref().unwrap();

    let paths = &state.user_data.app_paths;
    if !Path::exists(paths.user_data_path.as_path()) {
        println!(
            "Creating missing folders {:?}",
            paths.user_data_path.as_path()
        );
        _ = fs::create_dir_all(paths.user_data_path.as_path().parent().unwrap());
        _ = fs::write(paths.user_data_path.as_path(), "");
    }

    let data = fs::read(paths.user_data_path.as_path()).unwrap();
    let saved_data = dnd_protos::protos::UserData::decode(data.as_ref());
    if saved_data.is_err() {
        return Err(DiskError::DecodeError);
    }

    Ok(saved_data.unwrap())
}

/// Reads from APP_STATE
fn save_user_data_to_disk(data: dnd_protos::protos::UserData) -> Result<(), DiskError> {
    let state = crate::APP_STATE.read();
    if state.is_none() {
        return Err(DiskError::NoState);
    }

    let state = state.as_ref().unwrap();

    _ = fs::create_dir_all(state.user_data.app_paths.user_data_path.parent().unwrap());

    let mut buf = vec![];
    _ = data.encode(&mut buf);

    _ = fs::write(&state.user_data.app_paths.user_data_path, &buf);
    Ok(())
}

/// Reads from APP_STATE
pub fn save_current_user_data() -> Result<(), DiskError> {
    let state = crate::APP_STATE.read();
    if state.is_none() {
        return Err(DiskError::NoState);
    }

    let state = state.as_ref().unwrap();

    if let Some(sheet) = &state.user_data.sheet {
        let data = dnd_protos::protos::UserData {
            loaded_sheet: sheet.character_name.clone(),
        };

        save_user_data_to_disk(data)?;
        Ok(())
    } else {
        let data = dnd_protos::protos::UserData {
            loaded_sheet: String::new(),
        };

        save_user_data_to_disk(data)?;
        Ok(())
    }
}

pub fn load_sheet_from_path(path: &Path) -> Result<CharacterSheet, DiskError> {
    if !Path::is_file(path) {
        return Err(DiskError::FileNotFound);
    }

    let data = fs::read(path).unwrap();

    let saved_data = dnd_protos::protos::CharacterSheet::decode(data.as_ref());
    if saved_data.is_err() {
        return Err(DiskError::DecodeError);
    }

    Ok(saved_data.unwrap())
}

/// Reads from APP_STATE
pub fn load_current_sheet() -> Result<CharacterSheet, DiskError> {
    let disk_data = load_current_user_data()?;
    let state = crate::APP_STATE.read();
    if state.is_none() {
        return Err(DiskError::NoState);
    }

    let data = load_sheet_from_path(
        state
            .as_ref()
            .unwrap()
            .user_data
            .app_paths
            .sheet_path
            .join(disk_data.loaded_sheet)
            .as_path(),
    )?;
    Ok(data)
}

/// Reads from APP_STATE (paths)
pub fn save_sheet_to_disk(sheet: &CharacterSheet) -> Result<(), DiskError> {
    let mut buf = vec![];
    _ = sheet.encode(&mut buf);

    let state = crate::APP_STATE.read();
    if state.is_none() {
        return Err(DiskError::NoState);
    }

    let sheet_path = state
        .as_ref()
        .unwrap()
        .user_data
        .app_paths
        .sheet_path
        .join(sheet.character_name.clone());
    _ = fs::create_dir_all(sheet_path.parent().unwrap());
    _ = fs::write(sheet_path, &buf);

    Ok(())
}

/// Reads from APP_STATE (save_sheet_to_disk)
pub fn save_current_sheet() -> Result<(), DiskError> {
    let state = crate::APP_STATE.read();
    if state.is_none() {
        return Err(DiskError::NoState);
    }

    save_sheet_to_disk(state.as_ref().unwrap().user_data.sheet.as_ref().unwrap())
}
