use clap::{arg, Command, ArgMatches};
use crate::config::{Entry, parse};
use crate::actions::create;
use crate::Error;

// TODO - add commands for editing/initializing config
fn cli() -> ArgMatches {
    Command::new("pt")
        .about("Use a git repository as a template for your project")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
            .about("Initialize project")
            .arg(arg!(<TEMPLATE> "Name of template, specified in config.toml"))
            .arg(arg!(<NAME> "Target project name"))
            .arg_required_else_help(true)
        )
        .get_matches()
}


pub(crate) fn handle() -> Result<(), Error> {
    let config = parse()?;
    let args = cli();
    match args.subcommand() {
        Some(("init", sub_matches)) => {
            let template = sub_matches.get_one::<String>("TEMPLATE").unwrap();
            let target = sub_matches.get_one::<String>("NAME").unwrap();

            match config.repo.get(template) {
                Some(Entry{ source }) => {
                    if let Err(e) = create(source, target) {
                        return Err(e);
                    }
                },
                None => return Err(Error::MissingTemplate(template.to_owned())),
            };
        },

        _ => {}

    }

    Ok(())
}
