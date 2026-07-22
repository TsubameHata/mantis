//! Defines data models in project file, as well as project folder structure.
//! 
//! A Mantis project, at the current stage, 
//! consists of metadata, page images, vector masks and bitmap masks (, as well as slices produced, for future features ), 
//! which are packed into a folder,
//! where individual files are arranged into subfolders 
//! and the others (including those pointers to the individual files) are written in `json`.
//! 
//! The structure of the folder, as well as data model and explanation of files, is defined as below.
//! Folders and files need not always exist except `project.json`, and `pages.json` can exist with different name, which is referenced by `project.json`.
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