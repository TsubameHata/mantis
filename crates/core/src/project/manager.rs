//! Inside this file, it should be further considered whether to convert all `Result` into `io::Result`, or to define a result type.

use std::io;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::VERSION;
use crate::project::models::{OutputOptions, PagesFile, ProjectFile};

pub struct ProjectManager {
    base_path: PathBuf,
    pub project_file: ProjectFile,
    pub pages_file: PagesFile
}

/// Write `obj` into `file` as JSON string.
/// Designed for internal use, but set `pub` for examination.
/// 
/// `obj` must implement `serde::Serialize`.
/// 
/// `file` must be a file and in a valid directory.
pub fn write_json_to_file(obj: &impl Serialize, file: &Path) -> io::Result<()> {
    // Assert `file` to be a file, and its parent folder exists
    if !file.parent().ok_or(
        io::Error::new(
            io::ErrorKind::AddrNotAvailable,
            "The parent folder does not exist"
        )
    )?.try_exists().is_ok() {
        return Err(
            io::Error::new(
                io::ErrorKind::IsADirectory, 
                "Specified file is a directory"
            )
        );
    }
    
    fs::write(file, serde_json::to_string(&obj)
        .or(Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Cannot serialize given object"
        )))?
    )?;

    Ok(())
}

/// Read from `file` to construct a `T`.
pub fn read_json_from_file<T: DeserializeOwned>(file: &Path) -> io::Result<T> {
    let f = std::fs::File::open(file).ok().ok_or(io::Error::new(
        io::ErrorKind::Interrupted, 
        "Cannot open specified file"
    ))?;

    serde_json::from_reader(f).ok().ok_or(io::Error::new(
        io::ErrorKind::InvalidData, 
        "Cannot deserialize data"
    ))
}

impl ProjectManager {
    /// Creates a new project at specified path.
    /// 
    /// `base_path` is a folder to contain corresponding files.
    /// It is both okay for the folder to be existing or not.
    /// It is recommended for the folder to be blank if it exists.
    pub fn new(base_path: PathBuf) -> io::Result<Self> {
        if !base_path.try_exists().is_ok() {
            std::fs::create_dir_all(&base_path)?;
        } else {
            if !base_path.is_dir() {
                return Err(
                    io::Error::new(
                        io::ErrorKind::NotADirectory, 
                        "Not a directory"
                    )
                );
            }
        }

        let project_file = ProjectFile {
            mantis_version: semver::Version::parse(VERSION).unwrap(),
            pages: PathBuf::from("./pages.json"),
            output_options: OutputOptions::default()
        };

        let pages_file = PagesFile {
            pages: Vec::new()
        };

        let ret = ProjectManager {
            base_path,
            project_file,
            pages_file
        };

        ret.save_project_file()?;
        ret.save_pages_file()?;

        Ok(ret)
    }

    pub fn project_file_path(&self) -> PathBuf {
        self.base_path.join("./project.json")
    }

    pub fn pages_file_path(&self) -> PathBuf {
        self.base_path.join("./pages.json")
    }

    pub fn save_project_file(&self) -> io::Result<()> {
        write_json_to_file(&self.project_file, &self.project_file_path())?;
        Ok(())
    }

    pub fn save_pages_file(&self) -> io::Result<()> {
        write_json_to_file(&self.pages_file, &self.pages_file_path())?;
        Ok(())
    }
}