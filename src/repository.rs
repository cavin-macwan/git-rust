use clap::ArgMatches;
use std::fs::File;
use std::io::{Write, read_to_string};
use std::path::PathBuf;
use std::{env, fs, io};

use crate::git_object::GitObject;
use crate::git_object::ObjectType;

pub struct Repository {}

impl Repository {
    pub fn new() -> Self {
        Repository {}
    }

    pub fn get_current_working_dir() -> io::Result<PathBuf> {
        env::current_dir()
    }

    pub fn init(&self) -> bool {
        let git_path: PathBuf =
            Self::get_current_working_dir().unwrap_or_else(|_| PathBuf::from("."));

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
        return true;
    }

    pub fn add(&self, sub_matches: &ArgMatches) {
        let root_path: PathBuf =
            Self::get_current_working_dir().unwrap_or_else(|_| PathBuf::from("."));

        let git_dir = root_path.join(".rugit");

        if git_dir.exists() {
            let paths = sub_matches
                .get_many::<PathBuf>("PATH")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Adding {paths:?}");

            let file_path = root_path.join(paths.first().expect("Path is not provided"));
            match file_path.exists() {
                true => println!("File or Folder is present sir!"),
                false => {
                    println!("Bro could you please check the spelling while defining file")
                }
            }

            // check if the path we're trying to add is file or folder
            let is_file = file_path.is_file();
            match is_file {
                true => {
                    let content = fs::read(file_path).expect("Not able to read the content");
                    let blob = GitObject::new(ObjectType::Blob, content);
                    println!("Ready")
                }
                false => println!("We're in a folder"),
            }
        } else {
            println!(
                "You've not initialized the repository. Brother you should learn git before using it"
            )
        }
    }
}
