use std::fmt::{Display, Formatter};
use crate::cli::handle;

mod config;
mod cli;
mod actions;

// TODO improve error messages and handling
#[derive(Debug)]
pub(crate) enum Error {
    EnvError(String),
    IoError(std::io::Error),
    ParsingError,
    GitError(String),
    MissingTemplate(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::EnvError(name) => write!(f, "Failed to read environment variable \"{name}\""),
            Self::IoError(e) => e.fmt(f),
            Self::ParsingError => write!(f, "Failed to parse config file (check the corectness of config.toml?)"),
            Self::GitError(e) => e.fmt(f),
            Self::MissingTemplate(name) => write!(f, "Template \"{name}\" missing in config")
        }
    }
}

impl std::error::Error for Error {}

fn main() {
    if let Err(e) = handle() {
        println!("Error:\n{}", e)
    };

}
