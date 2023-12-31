use clap::{Parser,Subcommand,Args};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Configuration file location
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add or update config from list
    Modify(ModifyArgs),

    /// Remove program from list
    Remove(RemoveArgs),

    /// Print list of tracked configs
    List {},

    /// Update a config
    Sync(SyncArgs),
}

#[derive(Args)]
pub struct ModifyArgs {
    /// Name of program
    pub name: String,

    /// Parent directory on system config resides in
    pub dest: PathBuf,

    /// File(s) in configs directory
    pub file: PathBuf,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Name of program
    pub name: String,
}

#[derive(Args)]
pub struct SyncArgs {
    /// Names of programs to update
    pub names: Vec<String>,

    /// Sync all programs
    #[arg(short, long)]
    pub all: bool,

    /// Never prompt to skip
    #[arg(short, long)]
    pub force: bool,
}
