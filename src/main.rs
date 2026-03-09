use std::ffi::OsString;

use rust_tut::cli;

mod repository;
use repository::Repository;

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            Repository::init();
        }

        Some(("add", sub_matches)) => {
            Repository::add(sub_matches);
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
