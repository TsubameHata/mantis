//! Inside this file, it should be further considered whether to convert all `Result` into `io::Result`, or to define a result type.
//! 
//! A Mantis project, at the current stage, 
//! consists of metadata, page images, vector masks and bitmap masks (, as well as slices produced, for future features ), 
//! which are packed into a folder,
//! where individual files are arranged into subfolders 
//! and the others (including those pointers to the individual files) are written in `json`.
//! 
//! The structure of the folder, as well as data model and explanation of files, is defined as below.
//! Folders and files need not always exist except `project.json`, and `pages.json` can exist with different name, which is referenced by `project.json`.
//! All the reference paths are relative to the project root, which is a strict rule.
//! 
//! ```plain
//! project.mantis/
//!     project.json    (ProjectFile,   overall metadata, link to pages.json)
//!     pages.json      (PagesFile,      page metadata, links to page files, slice definition, vector masks, links to bitmap masks and output slices)
//!     pages/
//!         page-0001.png
//!         page-0002.png
//!         ...
//!         (naming need not strictly follow the convention above, since the pages are not referenced by naming convention, 
//!             but an increasing index is advised for readability)
//!     bitmap_masks/
//!         a39f62.png
//!         p28u7q.png
//!         ...
//!         (hash or uuid is advised for naming convention under this folder)
//!     outputs/
//!         page-0001-1.png
//!         page-0001-2.png
//!         ...
//!         (naming convention is not strict, and not advised at the current stage of development)
//! ``` 

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
    const PROJECT_FILE_PATH: &'static str = "./project.json";
    const PAGES_FILE_PATH: &'static str = "./pages.json";

    /// Creates a new project at specified path.
    /// 
    /// `base_path` is a folder to contain corresponding files.
    /// It is both okay for the folder to be existing or not.
    /// It is recommended for the folder to be blank if it exists.
    pub fn new(base_path: &Path) -> io::Result<Self> {
        if !base_path.try_exists().unwrap_or(false) {
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
            pages: PathBuf::from(Self::PAGES_FILE_PATH),
            output_options: OutputOptions::default()
        };

        let pages_file = PagesFile {
            pages: Vec::new()
        };

        let ret = Self {
            base_path: base_path.to_path_buf(),
            project_file,
            pages_file
        };

        ret.save_project_file()?;
        ret.save_pages_file()?;

        Ok(ret)
    }

    pub fn load(base_path: &Path) -> io::Result<Self> {
        if !base_path.is_dir() {
            return Err(
                io::Error::new(
                    io::ErrorKind::NotADirectory, 
                    "Not a directory"
                )
            );
        }
        
        let project_file = read_json_from_file(&base_path.join(Self::PROJECT_FILE_PATH))?;
        let pages_file = read_json_from_file(&base_path.join(Self::PAGES_FILE_PATH))?;

        Ok(
            Self {
                base_path: base_path.to_path_buf(),
                project_file,
                pages_file
            }
        )
    }

    pub fn project_file_path(&self) -> PathBuf {
        self.base_path.join(Self::PROJECT_FILE_PATH)
    }

    pub fn pages_file_path(&self) -> PathBuf {
        self.base_path.join(Self::PAGES_FILE_PATH)
    }

    pub fn save_project_file(&self) -> io::Result<()> {
        write_json_to_file(&self.project_file, &self.project_file_path())?;
        Ok(())
    }

    pub fn save_pages_file(&self) -> io::Result<()> {
        write_json_to_file(&self.pages_file, &self.pages_file_path())?;
        Ok(())
    }

    pub fn reload_project_file(&mut self) -> io::Result<()> {
        self.project_file = read_json_from_file(&self.project_file_path())?;
        Ok(())
    }

    pub fn reload_pages_file(&mut self) -> io::Result<()> {
        self.pages_file = read_json_from_file(&self.pages_file_path())?;
        Ok(())
    }
}