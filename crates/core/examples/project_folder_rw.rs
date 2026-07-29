use std::{path::PathBuf, str::FromStr};

use mantis::project::manager::ProjectManager;

fn main() {
    let base = PathBuf::from("./test_proj/");

    {
        let mut pm1 = ProjectManager::new(&base).unwrap();

        pm1.project_file.mantis_version = semver::Version::from_str("1.1.4").unwrap();
        pm1.save_project_file().unwrap();
    }

    {
        let pm2 = ProjectManager::load(&base).unwrap();
        dbg!(pm2.project_file.mantis_version);
    }
}