use crate::contract::Contract;
use std::collections::HashSet;
use std::fs::{DirEntry, ReadDir};
use std::io::ErrorKind;
use std::ops::Add;
use std::path::PathBuf;
use std::process::exit;
use std::{fs, io};

pub fn execute(namespace: PathBuf) {
    match fs::read_dir(namespace) {
        Ok(namespace) => {
            let mut checked_links: HashSet<PathBuf> = HashSet::new();
            if let Err(error) = recursive_output_methods(namespace, None, &mut checked_links) {
                eprintln!("IO Error: {error}");
                exit(1)
            }
        }
        Err(error) => {
            eprintln!("IO Error: {error}");
            exit(1)
        }
    }
}

fn recursive_output_methods(
    read_dir: ReadDir,
    object_name: Option<&str>,
    checked_links: &mut HashSet<PathBuf>,
) -> io::Result<()> {
    for file in read_dir {
        let file = &file?;

        if file.file_name().to_str().unwrap_or(".").contains('.') {
            continue;
        }

        if file.file_type()?.is_symlink() && fs::canonicalize(file.path())?.is_dir() {
            let dir = fs::canonicalize(file.path())?;

            if checked_links.insert(file.path()) {
                let object_name = {
                    if let Some(super_object) = object_name {
                        super_object
                            .to_string()
                            .add(".")
                            .add(file.file_name().to_str().unwrap())
                    } else {
                        file.file_name().to_str().unwrap().to_string()
                    }
                };

                recursive_output_methods(
                    fs::read_dir(dir)?,
                    Some(object_name.as_str()),
                    checked_links,
                )?;

                checked_links.remove(&file.path());

                continue;
            }
        }

        if file.file_type()?.is_dir() {
            let object_name = {
                if let Some(super_object) = object_name {
                    super_object
                        .to_string()
                        .add(".")
                        .add(file.file_name().to_str().unwrap())
                } else {
                    file.file_name().to_str().unwrap().to_string()
                }
            };

            if object_name.starts_with("__") && object_name.ends_with("__") {
                interface_output_methods(file, object_name);

                continue;
            }

            recursive_output_methods(
                fs::read_dir(file.path())?,
                Some(object_name.as_str()),
                checked_links,
            )?;
            continue;
        }

        if file.file_type()?.is_symlink() && !fs::canonicalize(file.path())?.is_file() {
            continue
        }

        if let Some(object) = object_name {
            if let Some(method) = file.file_name().to_str() {
                println!("{object}.{method}")
            }
        }
    }

    Ok(())
}

fn interface_output_methods(dir: &DirEntry, interface_name: String) {
    match fs::read_to_string(dir.path().join(".self")) {
        Ok(contracts) => {
            for line in contracts.lines() {
                match line.parse::<Contract>() {
                    Ok(contract) => {
                        println!("{interface_name}.{}", contract.name())
                    }
                    Err(_err) => {
                        eprintln!("Error while parsing contract: {line}");
                        exit(1)
                    }
                }
            }
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            eprintln!("Interfaces must contain a contracts file named \".self\"");
            exit(1)
        }
        Err(err) => {
            eprintln!("IO Error: {err}");
            exit(1)
        }
    }
}
