use std::{
    io::{self,ErrorKind,Write},
    fs,
    path::PathBuf,
};
use copy_dir::copy_dir;

use crate::{resolve_home,Config,Entry};

#[derive(Debug, PartialEq, Eq)]
enum ConflictOption {
    Remove,
    Rename,
    Skip,
}

fn remove_ambiguous(path: &PathBuf) -> io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(&path)?;
    } else {
        fs::remove_file(&path)?;
    }
    
    Ok(())
}

fn rename_conflict(path: &PathBuf) -> io::Result<()> {
    let rename_path = path.with_extension("old");

    if rename_path.try_exists()? {
        remove_ambiguous(&rename_path)?;
    }

    fs::rename(&path, &rename_path)?;
    println!("Old config renamed to {}.", rename_path.file_name().unwrap().to_str().unwrap());

    Ok(())
}

fn remove_existing(name: &str, path: &PathBuf) -> io::Result<Option<ConflictOption>> {
    loop {
        let mut input = String::new();

        print!("Remove existing config for {name}? (y/n/r[ename]) ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");

        let choice = match input.to_lowercase().trim() {
            "y" => ConflictOption::Remove,
            "n" => ConflictOption::Skip,
            "r" => ConflictOption::Rename,
            _ => {
                println!("Try again :)");
                continue;
            },
        };

        match choice {
            ConflictOption::Remove =>
                remove_ambiguous(path)?,
            ConflictOption::Rename =>
                rename_conflict(path)?,
            ConflictOption::Skip =>
                println!("Skipping {name}."),
        };

        return Ok(Some(choice));
    }
}

fn sync_file(
    name: &str,
    src: &PathBuf,
    dest: &PathBuf,
    force: &bool
) -> io::Result<Option<ConflictOption>> {

    let copy_result = copy_dir(src, dest);

    if let Err(error) = copy_result {
        if error.kind() == ErrorKind::AlreadyExists {
            if *force {
                remove_ambiguous(dest)?;
                return Ok(Some(ConflictOption::Remove));
            } else {
                return remove_existing(name, dest);
            }
        }
        return Err(error);
    }

    Ok(None)    
}

pub fn sync_config(name: &str, entry: &Entry, configs_path: &PathBuf, force: &bool) {
    let src = configs_path.join(&entry.file);
    let mut dest = entry.parent.join(&entry.file);
    resolve_home(&mut dest);

    loop {
        let sync_result = sync_file(name, &src, &dest, force);

        match sync_result {
            Ok(None) => println!("Updated {}.", dest.to_str().unwrap()),
            Ok(Some(choice)) => match choice {
                ConflictOption::Skip => break,
                _ => continue,
            },
            Err(error) => {
                eprintln!("Skipping sync of {src:?} to {dest:?}: {error:?}");
                break;
            },
        };

        break;
    }
}

pub fn sync_all(config: &Config, configs_path: &PathBuf, force: bool) {
    for (name, entry) in config {
        sync_config(&name, &entry, configs_path, &force);
    }
}
