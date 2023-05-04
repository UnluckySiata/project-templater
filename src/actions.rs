use std::{path::Path, fs::{remove_dir_all, remove_file}};
use git2::Repository;

use crate::Error;

pub(crate) fn create(source: &str, target: &str) -> Result<(), Error> {
    // TODO - let user decide what to exclude, provide defaults
    let ignored = [".gitignore", "LICENSE", "README.md", ".git"];

    let _ = match Repository::clone(source, target) {
        Ok(repo) => repo,
        Err(e) => return Err(Error::GitError(e.message().to_owned())),
    };

    for to_del in &ignored {
        let current = format!("{}/{}", target, to_del);
        let path = Path::new(&current);

        if path.exists() {
            if path.is_dir() {
                if let Err(e) = remove_dir_all(path) {
                    return Err(Error::IoError(e))
                };
            } else {
                if let Err(e) = remove_file(path) {
                    return Err(Error::IoError(e))
                };
            }
        }
    }

    let _ = match Repository::init(target) {
        Ok(repo) => repo,
        Err(e) => panic!("Couldn't initialize target repo: {}", e),
    };

    Ok(())

}

// TODO - add more features for transforming remplates
