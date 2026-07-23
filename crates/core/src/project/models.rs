//! Defines data models in project file, as well as project folder structure.
//! 
//! The models which are expected to have an individual file are named with `File` suffix,
//! and the models which are expected to be stored as fields are named directly as what they represent internally.
//! 
//! All structs in this file are expected to be converted to internal structs when read.
//! The structs in this file only represent their persistent storage models. Do not misunderstand their names and confuse them with those in other modules.
//! By the way, whether to create a model for some of the following fundamental structs without `impl` is still being considered, 
//! like `VectorShape` and `MaskValue`, while elevating them into `types.rs`.
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

use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use semver::Version;

use crate::types::{MaskValue, VectorShape, Overflow};

/// To be translated into settings of `ImageComposer`.
#[derive(Serialize, Deserialize, Default)]
pub struct OutputOptions {
    pub size: Option<(usize, usize)>,
    pub padding_y: Option<usize>,
    pub background_color: Option<(u8, u8, u8)>,
    pub overflow_x: Option<Overflow>,
    pub overflow_y: Option<Overflow>
}

#[derive(Serialize, Deserialize)]
pub struct ProjectFile {
    pub mantis_version: Version,
    pub pages_file: PathBuf,

    #[serde(default)]
    pub output_options: OutputOptions
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MaskSource {
    File(PathBuf),
    Vector(VectorShape)
}

#[derive(Serialize, Deserialize)]
pub struct Mask {
    pub value: MaskValue,
    pub shape: MaskSource
}

#[derive(Serialize, Deserialize)]
pub struct Slice {
    pub y_padding: Option<(usize, usize)>,
    pub output: Option<PathBuf>,
    pub masks: Vec<Mask>
}

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub image: PathBuf,
    pub slices: Vec<Slice>
}

#[derive(Serialize, Deserialize)]
pub struct PagesFile {
    pub pages: Vec<Page>
}