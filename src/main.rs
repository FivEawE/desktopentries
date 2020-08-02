use crate::checker::Checker;
use crate::configuration::Configuration;
use crate::entry::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::{
    env,
    io::{self, BufWriter, Write},
};
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
                let mut path = PathBuf::from(dir);
                path.push("applications/");

                get_entries_from_path(&path.display().to_string(), path.as_path(), &mut entries);
            }
        }
        Err(_) => {
            let base_path = "/usr/share/applications/";
            eprintln!("$XDG_DATA_DIRS not set, defaulting to {}", base_path);

            get_entries_from_path(&String::from(base_path), Path::new(base_path), &mut entries);
        }
    }

    let stdout = io::stdout();
    let mut out_handle = BufWriter::new(stdout);

    for entry_path in entries.values() {
        let contents = fs::read_to_string(entry_path);
        match contents {
            Ok(contents) => {
                let entry = Entry::new(&contents);
                match entry {
                    Ok(entry) => {
                        if checker.check_entry(&entry) {
                            let out = writeln!(out_handle, "{}", entry_path.display());
                            match out {
                                Ok(_) => {
                                    let out = writeln!(out_handle, "{}", contents);
                                    if let Err(_) = out {
                                        eprintln!("Error while outputting to stdout");
                                    }
                                }
                                Err(_) => eprintln!("Error while outputting to stdout"),
                            }
                        }
                    }
                    Err(error) => eprintln!("{} in {}", error, entry_path.display()),
                }
            }
            Err(_) => eprintln!("Could not read file {}", entry_path.display()),
        }
    }
}

fn get_entries_from_path(base_path: &String, path: &Path, entries: &mut HashMap<String, PathBuf>) {
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
                                        let key =
                                            file_name.replace(base_path, "").replace('/', "-");
                                        entries.entry(key).or_insert(file.path());
                                    }
                                }
                            }
                            Err(_) => eprintln!("Could not get file type of {}", file.path()
                                .display()),
                        }
                    }
                    Err(_) => eprintln!("There was an error while iterating over folder contents \
                    of {}", path.display()),
                }
            }
        }
        Err(_) => eprintln!("Could not read path {}", path.display()),
    }
}
