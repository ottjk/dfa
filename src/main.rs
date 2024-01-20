use std::{
    fs,
    io::Result,
    collections::HashMap,
    path::PathBuf,
};
use serde::{Deserialize,Serialize};
use clap::Parser;

mod commands;
use commands::*;

mod arguments;
use arguments::*;

#[derive(Deserialize, Serialize)]
pub struct Entry {
    file: PathBuf,
    parent: PathBuf,
}

pub type Config = HashMap<String, Entry>;

pub fn resolve_home(path: &mut PathBuf) {
    if path.starts_with("~") {
        let temp = path.strip_prefix("~").unwrap();
        *path = dirs::home_dir().expect("Could not resolve home directory.")
            .join(temp);
    }
}

fn read_config(path: &PathBuf) -> Config {
    let file = fs::read_to_string(path).unwrap_or(String::new());
    toml::from_str(&file).unwrap()
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut configs_path = cli.config
        .unwrap_or(dirs::home_dir()
                   .expect("Could not resolve home directory.")
                   .join(".local/share/dotfiles"));
    resolve_home(&mut configs_path);

    if !configs_path.try_exists().unwrap() {
        fs::create_dir_all(&configs_path).unwrap();
    }

    let mut config = read_config(&configs_path.join("configs.toml"));

    match cli.command {
        Commands::Modify(args) => modify_command(&mut config, args, &configs_path)?,
        Commands::Remove(args) => remove_command(&mut config, args, &configs_path)?,
        Commands::Sync(args) => sync_command(&config, args, &configs_path),
        Commands::List {} => list_command(&config),
    };

    Ok(())
}
