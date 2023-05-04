
# project-templater (pt)

## Introduction
Project-templater is a simple tool for initializing projects with the use of
predefined templates in form of any git repository. It clones the template,
purges it of unwanted files and initializes an empty git repository.
The features and customization is currently very limited, but there's more
to come in the near future.

## Installation
The project is in it's very early version, so for now the only way to install
it is from the main branch of this repo.
The command below will do just that (assuming that you have cargo installed)
```cargo install --git https://github.com/unluckysiata/project-templater```

## Usage
After calling the binary without any subcommands, you will see the following menu
```
Use a git repository as a template for your project

Usage: pt <COMMAND>

Commands:
  init  Initialize project
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
If you want help for specific subcommand, use `pt <COMMAND> --help`
For initializing a project just enter
```pt init <template name> <repository destination>```

## Configuration
In order to use, either create a *config.toml* inside *$HOME/.config/project-templater*
or let the bare config be created for you after the first usage. The config file
should have the following structure
```toml
# Mandatory part
[repo]
# Your templates
# For example this is the template aliased as "name" sourced from
# github repository user/example (but any git repo, remote or local, is ok)
[repo.name]
source = https://github.com/user/example
```




