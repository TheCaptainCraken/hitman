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

        #[arg(short, long, default_value_t = false)]
        /// Show the name of every file kidnapped.
        verbose: bool,
    },

    #[command()]
    /// Restores a file or folder to their old location
    Free {
        /// File or folder to restore.
        target: OsString,

        #[arg(short, long, default_value = ".")]
        /// New location for the file instead of the older
        new_location: OsString,

        #[arg(short, long, default_value_t = false)]
        /// Show the name of every file freed.
        verbose: bool,
    },

    #[command()]
    /// List the files that can be restored.
    ListKidnapped {
        #[arg(short, long, default_value_t = false)]
        /// Show the name of every file and subfolders.
        recursive: bool,
    },

    #[command()]
    /// Deletes the contents of the recovery folder permanently.
    Massacre {
        #[arg(short, long, default_value_t = false)]
        /// Show the name of every file massacred.
        verbose: bool,
    },
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    match args.command {
        Command::Massacre { verbose } => massacre::handle_massacre(verbose)?,
        Command::Kidnap {
            target,
            kill,
            verbose,
        } => kidnap::handle_kidnap(target, kill, verbose)?,
        Command::Free {
            target,
            new_location,
            verbose,
        } => free::free(target, new_location, verbose)?,
        Command::ListKidnapped { recursive } => kidnap::list_kidnapped(recursive)?,
    };

    Ok(())
}
