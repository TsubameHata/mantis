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
    pub pages: PathBuf,

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