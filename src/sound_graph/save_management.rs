#[cfg(target_arch = "wasm32")]
use rfd::{AsyncFileDialog, FileHandle};
#[cfg(not(target_arch = "wasm32"))]
use rfd::{FileDialog, FileHandle};

use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use super::graph::SoundGraphEditorState;

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
    pub graph_state: SoundGraphEditorState,
}

pub fn get_current_working_settings(
    app_path: &str,
) -> Result<WorkingFileSettings, Box<dyn std::error::Error>> {
    if std::path::Path::new(app_path).exists() {
        return Ok(ron::de::from_str(fs::read_to_string(app_path)?.as_str())?);
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

async fn convert_file_handle(
    fh_op: Option<FileHandle>,
) -> Result<String, Box<dyn std::error::Error>> {
    let fh = match fh_op {
        None => {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "file pick did not work",
            )));
        }
        Some(x) => x,
    };
    let path = fh.read().await;
    Ok(String::from_utf8(path)?)
}

pub fn save_project_file(
    project_file: ProjectFile,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, ron::ser::to_string(&project_file)?)?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub async fn save_project_file_as_async(
    project_file: ProjectFile,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = convert_file_handle(
        AsyncFileDialog::new()
            .add_filter("text", &["ron"])
            .set_directory("./")
            .save_file()
            .await,
    )
    .await?;
    save_project_file(project_file, &path)?;
    Ok(path)
}

#[cfg(target_arch = "wasm32")]
pub async fn open_project_file_async() -> Result<(String, ProjectFile), Box<dyn std::error::Error>>
{
    let file = convert_file_handle(
        AsyncFileDialog::new()
            .add_filter("text", &["ron"])
            .set_directory("./")
            .pick_file()
            .await,
    )
    .await?;
    Ok((file.clone(), get_project_file(file.as_str())?))
}
#[cfg(target_arch = "wasm32")]
pub async fn set_output_sound_destination_async() -> Result<String, Box<dyn std::error::Error>> {
    let file = convert_file_handle(
        AsyncFileDialog::new()
            .add_filter("audio", &["wav"])
            .set_directory("./")
            .save_file()
            .await,
    )
    .await?;
    Ok(file)
}

#[cfg(target_arch = "wasm32")]
pub async fn set_input_sound_destination_async() -> Result<String, Box<dyn std::error::Error>> {
    let file = convert_file_handle(
        AsyncFileDialog::new()
            .add_filter("sound", &["ogg", "mp3", "wav"])
            .set_directory("./")
            .pick_file()
            .await,
    )
    .await?;
    Ok(file)
}

pub fn get_project_file(path: &str) -> Result<ProjectFile, Box<dyn std::error::Error>> {
    Ok(ron::de::from_str(&fs::read_to_string(path)?)?)
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
pub fn open_project_file() -> Result<(String, ProjectFile), Box<dyn std::error::Error>> {
    let file = convert_option_pathbuf(
        FileDialog::new()
            .add_filter("text", &["ron"])
            .set_directory("./")
            .pick_file(),
    )?;
    Ok((file.clone(), get_project_file(file.as_str())?))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_output_sound_destination() -> Result<String, Box<dyn std::error::Error>> {
    let file = convert_option_pathbuf(
        FileDialog::new()
            .add_filter("audio", &["wav"])
            .set_directory("./")
            .save_file(),
    )?;
    Ok(file)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_input_sound_destination() -> Result<String, Box<dyn std::error::Error>> {
    let file = convert_option_pathbuf(
        FileDialog::new()
            .add_filter("sound", &["ogg", "mp3", "wav"])
            .set_directory("./")
            .pick_file(),
    )?;
    Ok(file)
}
