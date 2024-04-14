use std::{fs, path::Path};

use dnd_protos::protos::{CharacterSheet, UserData};
use prost::Message;

#[derive(Debug)]
pub enum DiskError {
    NoState,
    DecodeError,
    FileNotFound,
}

pub fn load_disk_data() -> Result<UserData, DiskError> {
    let state = crate::APP_STATE.read().unwrap();
    if state.is_none() {
        return Err(DiskError::NoState);
    }

    let state = state.as_ref().unwrap();

    let paths = &state.user_data.app_paths;
    if !Path::exists(paths.user_data_path.as_path()) {
        println!("Creating missing folders {:?}", paths.user_data_path.as_path());
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

fn save_user_data(data: dnd_protos::protos::UserData) -> Result<(), DiskError> {
    println!("Writing to disk");
    let state = crate::APP_STATE.read().unwrap();
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

pub fn save_disk_data() -> Result<(), DiskError> {
    let state = crate::APP_STATE.read().unwrap();
    if state.is_none() {
        return Err(DiskError::NoState);
    }

    let state = state.as_ref().unwrap();

    if let Some(sheet) = &state.user_data.sheet {
        let data = dnd_protos::protos::UserData {
            loaded_sheet: sheet.character_name.clone(),
        };

        save_user_data(data)?;
        Ok(())
    } else {
        let data = dnd_protos::protos::UserData {
            loaded_sheet: String::new(),
        };

        save_user_data(data)?;
        Ok(())
    }
}

pub fn load_sheet() -> Result<CharacterSheet, DiskError> {
    let disk_data = load_disk_data()?;
    let state = crate::APP_STATE.read().unwrap();
    if state.is_none() {
        return Err(DiskError::NoState);
    }

    if !Path::is_file(
        state
            .as_ref()
            .unwrap()
            .user_data
            .app_paths
            .sheet_path
            .join(&disk_data.loaded_sheet)
            .as_path(),
    ) {
        return Err(DiskError::FileNotFound);
    }

    let data = fs::read(
        state
            .as_ref()
            .unwrap()
            .user_data
            .app_paths
            .sheet_path
            .join(&disk_data.loaded_sheet)
            .as_path(),
    )
    .unwrap();

    drop(state);

    let saved_data = dnd_protos::protos::CharacterSheet::decode(data.as_ref());
    if saved_data.is_err() {
        return Err(DiskError::DecodeError);
    }

    Ok(saved_data.unwrap())
}

pub fn save_sheet(sheet: &CharacterSheet) -> Result<(), DiskError> {
    let mut buf = vec![];
    _ = sheet.encode(&mut buf);

    let state = crate::APP_STATE.read().unwrap();
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

pub fn save_current_sheet() -> Result<(), DiskError> {
    let state = crate::APP_STATE.read().unwrap();
    if state.is_none() {
        return Err(DiskError::NoState);
    }

    save_sheet(state.as_ref().unwrap().user_data.sheet.as_ref().unwrap())
}
