//! Defines data models in project file.
//! 
//! A Mantis project, temporarily, 
//! consists of metadata, page images, vector masks and bitmap masks, which are packed into a folder,
//! where individual files are arranged into subfolders and 
//! the others (including those pointers to the individual files) are written in `json`.
//! 
//! The structure of the folder is defined as below: