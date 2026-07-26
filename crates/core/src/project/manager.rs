use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::VERSION;
use crate::project::models::{OutputOptions, PagesFile, ProjectFile};

pub struct ProjectManager {
    base_path: PathBuf,
    pub project_file: ProjectFile,
    pub pages_file: PagesFile
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
        todo!()
    }

    pub fn save_pages_file(&self) -> io::Result<()> {
        todo!()
    }
}