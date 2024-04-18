use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use rfd::FileDialog;
use serde::{Deserialize, Serialize};

use super::graph::SoundNodeGraphSavedState;

pub fn get_current_exe_dir() -> Option<String> {
    Some(
        std::path::Path::new(std::env::current_exe().unwrap().to_str().unwrap())
            .parent()?
            .to_str()?
            .to_string(),
    )
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkingFileSettings {
    pub latest_saved_file: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectFile {
    pub graph_state: SoundNodeGraphSavedState,
}

pub fn get_current_working_settings(
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

pub fn save_current_working_settings(
    app_path: &str,
    settings: WorkingFileSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(fs::write(app_path, ron::ser::to_string(&settings)?)?)
}

pub fn convert_option_pathbuf(
    path_buf_op: Option<PathBuf>,
) -> Result<String, Box<dyn std::error::Error>> {
    let path_buf = match path_buf_op {
        None => {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "file pick did not work",
            )));
        }
        Some(x) => x,
    };
    let path = match path_buf.into_os_string().into_string() {
        Ok(x) => x,
        Err(_) => {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "conversion into string from os string failed",
            )))
        }
    };
    Ok(path)
}

pub fn save_project_file_as(
    project_file: ProjectFile,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = convert_option_pathbuf(
        FileDialog::new()
            .add_filter("text", &["ron"])
            .set_directory("./")
            .save_file(),
    )?;
    save_project_file(project_file, &path)?;
    Ok(path)
}

pub fn save_project_file(
    project_file: ProjectFile,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, ron::ser::to_string(&project_file)?)?;
    Ok(())
}

pub fn open_project_file() -> Result<ProjectFile, Box<dyn std::error::Error>> {
    get_project_file(
        convert_option_pathbuf(
            FileDialog::new()
                .add_filter("text", &["ron"])
                .set_directory("./")
                .pick_file(),
        )?
        .as_str(),
    )
}

pub fn get_project_file(path: &str) -> Result<ProjectFile, Box<dyn std::error::Error>> {
    Ok(ron::de::from_str(&fs::read_to_string(path)?)?)
}
