use std::ffi::OsString;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use rust_tut::cli;

fn get_current_working_dir() -> io::Result<PathBuf> {
    env::current_dir()
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            let git_path: PathBuf =
                get_current_working_dir().unwrap_or_else(|_| PathBuf::from("."));

            let git_dir = git_path.join(".rugit");
            let objects_dir = git_dir.join("objects");

            fs::create_dir_all(&git_dir).expect("Failed to create .rugit directory");
            fs::create_dir_all(&objects_dir).expect("Failed to create .rugit directory");

            let head_path = git_dir.join("HEAD");
            let index_path = git_dir.join("INDEX");

            File::create(&head_path).expect("Failed to create HEAD file");
            File::create(&index_path).expect("Failed to create INDEX file");
        }

        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {ext:?} with {args:?}");
        }

        _ => unreachable!(),
    }
}
