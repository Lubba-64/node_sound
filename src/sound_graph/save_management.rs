use std::{
    fs,
    io::{Error, ErrorKind},
};

use rfd::FileDialog;
use serde::{Deserialize, Serialize};

fn get_current_exe_dir() -> Option<String> {
    Some(
        std::path::Path::new(std::env::current_exe().unwrap().to_str().unwrap())
            .parent()?
            .to_str()?
            .to_string(),
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WorkingFileSettings {
    latest_saved_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {}

fn get_current_working_settings(
    app_path: &str,
) -> Result<WorkingFileSettings, Box<dyn std::error::Error>> {
    let settings_file = format!("{}/settings.ron", app_path);
    if std::path::Path::new(settings_file.as_str()).exists() {
        return Ok(ron::de::from_str(
            fs::read_to_string(settings_file)?.as_str(),
        )?);
    } else {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "main settings file does not exist",
        )));
    }
}

fn write_current_working_settings(
    app_path: &str,
    settings: WorkingFileSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(fs::write(app_path, ron::ser::to_string(&settings)?)?)
}

fn save_current_as(data: ProjectFile) -> Option<String> {
    let file = FileDialog::new()
        .add_filter("text", &[".ron"])
        .set_directory("./")
        .pick_file()?
        .to_str()?;
    Some("".to_string())
}
