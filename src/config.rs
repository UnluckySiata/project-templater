use std::env::var_os;
use std::ffi::{OsString, OsStr};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) repo: HashMap<String, Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Entry {
    pub(crate) source: String,
}

fn init_config() -> Result<PathBuf, Error> {
    let target: OsString = OsStr::new(".config/project-templater").into();
    let home_var: String = "HOME".into();
    let mut path: PathBuf = match var_os(&home_var) {
        Some(val) => [val, target].iter().collect(),
        None => return Err(Error::EnvError(home_var)),
    };

    if let Err(e) = fs::create_dir_all(&path) {
        return Err(Error::IoError(e))
    }
    path.push("config.toml");

    if !path.exists() {
        let example = "\
            [repo] \n\
            # Here's a way to add a template aliased \"name\" from github \n\
            # [repo.name] \n\
            # source = https://github.com/user/template-name \n\
            ";
        if let Err(e) = fs::write(&path, example) {
            return Err(Error::IoError(e))
        }
    }
    Ok(path)
}

pub(crate) fn parse() -> Result<Config, Error> {
    let path = init_config()?;

    let config = match std::fs::read_to_string(path) {
        Ok(val) => val,
        Err(e) => return Err(Error::IoError(e)),
    };
    match toml::from_str(&config) {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(Error::ParsingError),
    }
}
