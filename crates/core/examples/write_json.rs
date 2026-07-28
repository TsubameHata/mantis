use std::path::PathBuf;
use std::str::FromStr;

use mantis::VERSION;
use mantis::project::models::{OutputOptions, ProjectFile};
use mantis::project::manager::write_json_to_file;

fn main() {
    let proj_test = ProjectFile {
        mantis_version: semver::Version::from_str(&VERSION).unwrap(),
        pages: PathBuf::new(),
        output_options: OutputOptions::default()
    };

    write_json_to_file(proj_test, &PathBuf::from("./test.json")).unwrap();
}