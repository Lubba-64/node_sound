#[cfg(target_arch = "wasm32")]
use futures::Future;
#[cfg(target_arch = "wasm32")]
use futures::FutureExt;
#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
#[cfg(target_arch = "wasm32")]
use rfd::{AsyncFileDialog, FileHandle};
#[cfg(target_arch = "wasm32")]
use std::cell::Cell;
#[cfg(target_arch = "wasm32")]
use std::panic;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use std::thread;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures;

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

pub fn save_project_file(
    project_file: ProjectFile,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, ron::ser::to_string(&project_file)?)?;
    Ok(())
}

pub fn get_project_file(path: &str) -> Result<ProjectFile, Box<dyn std::error::Error>> {
    Ok(ron::de::from_str(&fs::read_to_string(path)?)?)
}

pub use open_project_file::open_project_file;

#[cfg(target_arch = "wasm32")]
pub struct Task<T>(Rc<Cell<Option<thread::Result<T>>>>);

#[cfg(target_arch = "wasm32")]
impl<T: 'static> Task<T> {
    pub fn spawn<F: 'static + Future<Output = T>>(future: F) -> Self {
        let sender = Rc::new(Cell::new(None));
        let receiver = sender.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let future = panic::AssertUnwindSafe(future).catch_unwind();
            sender.set(Some(future.await));
        });
        Self(receiver)
    }
    pub fn take_output(&self) -> Option<thread::Result<T>> {
        self.0.take()
    }
}

#[cfg(target_arch = "wasm32")]
async fn convert_file_handle(
    fh_op: Option<FileHandle>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let fh = match fh_op {
        None => {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "file pick did not work",
            )));
        }
        Some(x) => x,
    };
    Ok((String::from_utf8(fh.read())?, fh.name()))
}

#[cfg(target_arch = "wasm32")]
fn do_wasm_task<T: 'static, F: Future<Output = Result<T, Box<dyn std::error::Error>>> + 'static>(
    future: F,
) -> Result<T, Box<dyn std::error::Error>> {
    let path = match Task::<Result<T, Box<dyn std::error::Error>>>::spawn(future).take_output() {
        Some(x) => x,
        None => {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "file pick did not work",
            )));
        }
    };
    let project_file_unwrapped = match path {
        Err(x) => {
            return Err(Box::new(Error::new(
                ErrorKind::Other,
                "file pick did not work",
            )));
        }
        Ok(x) => x,
    }?;
    return Ok(project_file_unwrapped);
}

mod open_project_file {
    use super::*;
    #[cfg(not(target_arch = "wasm32"))]
    fn open_project_file_sync() -> Result<(String, ProjectFile), Box<dyn std::error::Error>> {
        let file = convert_option_pathbuf(
            FileDialog::new()
                .add_filter("text", &["ron"])
                .set_directory("./")
                .pick_file(),
        )?;
        Ok((file.clone(), get_project_file(file.as_str())?))
    }

    #[cfg(target_arch = "wasm32")]
    async fn open_project_file_async() -> Result<(String, ProjectFile), Box<dyn std::error::Error>>
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

    pub fn open_project_file(
    ) -> Result<(std::string::String, ProjectFile), Box<(dyn std::error::Error)>> {
        #[cfg(target_arch = "wasm32")]
        {
            do_wasm_task(open_project_file_async())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            open_project_file_sync()
        }
    }
}

pub use output_sound_destination::set_output_sound_destination;

mod output_sound_destination {
    use super::*;
    #[cfg(not(target_arch = "wasm32"))]
    fn set_output_sound_destination_sync() -> Result<String, Box<dyn std::error::Error>> {
        let file = convert_option_pathbuf(
            FileDialog::new()
                .add_filter("audio", &["wav"])
                .set_directory("./")
                .save_file(),
        )?;
        Ok(file)
    }

    #[cfg(target_arch = "wasm32")]
    async fn set_output_sound_destination_async() -> Result<String, Box<dyn std::error::Error>> {
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

    pub fn set_output_sound_destination(
    ) -> Result<std::string::String, Box<(dyn std::error::Error)>> {
        #[cfg(target_arch = "wasm32")]
        {
            do_wasm_task(set_output_sound_destination_async())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            set_output_sound_destination_sync()
        }
    }
}

pub use input_sound_destination::set_input_sound_destination;

mod input_sound_destination {
    use super::*;
    #[cfg(not(target_arch = "wasm32"))]
    fn set_input_sound_destination_sync() -> Result<String, Box<dyn std::error::Error>> {
        let file = convert_option_pathbuf(
            FileDialog::new()
                .add_filter("sound", &["ogg", "mp3", "wav"])
                .set_directory("./")
                .pick_file(),
        )?;
        Ok(file)
    }

    #[cfg(target_arch = "wasm32")]
    async fn set_input_sound_destination_async() -> Result<String, Box<dyn std::error::Error>> {
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

    pub fn set_input_sound_destination() -> Result<std::string::String, Box<(dyn std::error::Error)>>
    {
        #[cfg(target_arch = "wasm32")]
        {
            do_wasm_task(set_input_sound_destination_async())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            return set_input_sound_destination_sync();
        }
    }
}

pub use save_project_file_as::save_project_file_as;

mod save_project_file_as {
    use super::*;
    #[cfg(not(target_arch = "wasm32"))]
    fn save_project_file_as_sync(
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

    #[cfg(target_arch = "wasm32")]
    async fn save_project_file_as_async(
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
        Ok(path.1)
    }

    pub fn save_project_file_as(
        project_file: ProjectFile,
    ) -> Result<String, Box<dyn std::error::Error>> {
        #[cfg(target_arch = "wasm32")]
        {
            do_wasm_task(save_project_file_as_async(project_file))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            save_project_file_as_sync(project_file)
        }
    }
}
