use crate::checker::Checker;
use crate::configuration::Configuration;
use crate::entry::Entry;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod checker;
mod configuration;
mod entry;

fn main() {
    let conf = Configuration::from_args();
    let checker = Checker::new(conf);

    let mut entries = HashMap::new();
    let xdg_data_dirs = env::var("XDG_DATA_DIRS");
    match xdg_data_dirs {
        Ok(value) => {
            for dir in value.split(':') {
                get_entries_from_path(dir, Path::new(dir), &mut entries);
            }
        }
        Err(_) => {
            let base_path = "/usr/share/applications";
            eprintln!("$XDG_DATA_DIRS not set, defaulting to {}", base_path);

            get_entries_from_path(base_path, Path::new(base_path), &mut entries);
        }
    }

    for entry_tuple in entries {
        let contents = fs::read_to_string(&entry_tuple.1);
        match contents {
            Ok(contents) => {
                let entry = Entry::new(&contents);
                match entry {
                    Ok(entry) => {
                        if checker.check_entry(&entry) {
                            println!("{}", entry_tuple.1.display());
                            println!("{}", contents);
                            println!();
                        }
                    }
                    Err(error) => eprintln!("{}", error),
                }
            }
            Err(error) => eprintln!("{}", error),
        }
    }
}

fn get_entries_from_path(base_path: &str, path: &Path, entries: &mut HashMap<String, PathBuf>) {
    let dir_iterator = fs::read_dir(path);
    match dir_iterator {
        Ok(dir_iterator) => {
            for file in dir_iterator {
                match file {
                    Ok(file) => {
                        let file_type = file.file_type();
                        match file_type {
                            Ok(file_type) => {
                                if file_type.is_dir() {
                                    get_entries_from_path(
                                        base_path,
                                        file.path().as_path(),
                                        entries,
                                    );
                                } else if file_type.is_file() {
                                    let file_name = file.path().display().to_string();
                                    if file_name.ends_with(".desktop") {
                                        let key = String::from(&file_name[base_path.len()..])
                                            .replace('/', "-");
                                        entries.entry(key).or_insert(file.path());
                                    }
                                }
                            }
                            Err(error) => eprintln!("{}", error),
                        }
                    }
                    Err(error) => eprintln!("{}", error),
                }
            }
        }
        Err(error) => eprintln!("{}", error),
    }
}
