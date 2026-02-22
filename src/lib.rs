use clap::{Command, arg};

pub fn cli() -> Command {
    Command::new("git")
        .about("This is cavin's git")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a new, empty repository.")
                .long_about("Initialize a new, empty repository.\n\nThis command creates an empty Git repository - basically a .git directory with subdirectories for objects, refs/heads, refs/tags, and template files. An initial HEAD file is created pointing to the master branch.\n\nIf the $GIT_DIR environment variable is set, its value is used as the location of the repository. Otherwise, the repository is initialized in the current working directory unless a path is specified via the <GIT_PATH> argument.")
                .arg(
                    arg!(<GIT_PATH> "The path to initialize the repository at")
                        .required(false)
                        .default_value("."),
                ),
        )
}
