use clap::{Parser, Subcommand};
use std::ffi::OsString;

mod config;
mod free;
mod kidnap;
mod massacre;
mod utilities;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command()]
    /// Safely removes a file or a list of files.
    Kidnap {
        /// File or folder to remove.
        target: OsString,

        #[arg(long, default_value_t = false)]
        /// Permanently deletes the files instead of moving them.
        kill: bool,
    },

    #[command()]
    /// Restores a file or folder to their old location
    Free {
        /// File or folder to restore.
        target: OsString,

        #[arg(short, long)]
        /// New location for the file instead of the older
        new_location: Option<OsString>,
    },

    #[command()]
    /// List the files that can be restored.
    ListHostages,

    #[command()]
    /// Deletes the contents of the recovery folder permanently.
    Massacre,
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    match args.command {
        Command::Massacre => massacre::handle_massacre()?,
        Command::Kidnap { target, kill } => kidnap::handle_kidnap(target, kill)?,
        Command::Free {
            target,
            new_location,
        } => free::free(target, new_location)?,
        Command::ListHostages => kidnap::list_hostages()?,
    };

    Ok(())
}
