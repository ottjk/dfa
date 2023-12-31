use std::{
    fs,
    io::{self,Result,Write},
    path::PathBuf,
};

mod sync;

use crate::{Config,Entry};
use crate::arguments::*;

fn save_config(config: &Config, path: &PathBuf) -> Result<()> {
    let toml = toml::to_string(config).unwrap();
    fs::write(path.join("configs.toml"), toml)?;
    Ok(())
}

pub fn modify_command(
    config: &mut Config,
    args: ModifyArgs,
    configs_path: &PathBuf,
) -> Result<()> {

    if config.contains_key(&args.name) { loop {
        let mut input = String::new();

        print!("Entry already exists for {}. Overwrite it? (y/n) ", &args.name);
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");

        match input.to_lowercase().trim() {
            "y" => break,
            "n" => return Ok(()),
            _ => {
                println!("Try again :)");
                continue;
            },
        };
    }}

    let entry = Entry { file: args.file, parent: args.dest };    
    config.insert(args.name, entry);
    save_config(&config, configs_path)?;

    Ok(())
}

pub fn remove_command(
    config: &mut Config,
    args: RemoveArgs,
    configs_path: &PathBuf
) -> Result<()> {

    if !config.contains_key(&args.name) {
        panic!("Such an entry does not exist.");
    }
    config.remove(&args.name);
    save_config(&config, configs_path)?;

    Ok(())
}

pub fn list_command(config: &Config) {
    for (key,entry) in config {
        println!(r#""{}" - src: {}, dest: {}"#, key, entry.file.to_str().unwrap(), entry.parent.to_str().unwrap());
    }
}

pub fn sync_command(config: &Config, args: SyncArgs, configs_path: &PathBuf) {
    if args.all {
        sync::sync_all(config, configs_path, args.force);
        return
    }

    for name in &args.names {
        let entry = match config.get(name) {
            Some(e) => e,
            None => {
                eprintln!(r#"Entry "{name}" not found."#);
                continue;
            },
        };

        sync::sync_config(name, entry, configs_path, &args.force);
    }
}
