use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
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
            let ref_dir = git_dir.join("refs");
            let heads_dir = ref_dir.join("heads");

            fs::create_dir_all(&git_dir).expect("Failed to create .rugit directory");
            fs::create_dir_all(&objects_dir).expect("Failed to create .objects directory");
            fs::create_dir_all(&ref_dir).expect("Failed to create .refs directory");
            fs::create_dir_all(&heads_dir).expect("Failed to create .heads directory");

            let head_path = git_dir.join("HEAD");
            let index_path = git_dir.join("INDEX");

            let mut head_file = File::create(&head_path).expect("Failed to create HEAD file");
            let mut index_file = File::create(&index_path).expect("Failed to create INDEX file");

            head_file
                .write_all(b"ref: refs/heads/main\n")
                .expect("Failed to write HEAD file");

            index_file
                .write_all(b"{}")
                .expect("Failed to write index file");

            println!("Your amazing repository has been initialized successfully");
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
