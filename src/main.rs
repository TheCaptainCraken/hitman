use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command()]
    /// Removes a file or a list of files.
    /// It also removes folders and it's recursive.
    /// This is safe meaning the files don't get deleted but just moved to a folder known as 'the basement'.
    Kidnap {
        /// File or folder to remove.
        targets: std::ffi::OsString,

        #[arg(long, default_value_t = false)]
        /// Permanently removes the files instead of moving them.
        kill: bool,

        #[arg(short, long, default_value = None)]
        /// List of files and/or subfolders to ignore.
        spare: Option<Vec<std::ffi::OsString>>,

        #[arg(short, long, default_value_t = false)]
        /// Show the name of every file kidnapped.
        verbose: bool,
    },

    #[command()]
    /// Restores a file or folder to their old location
    Free {
        /// File or folder to restore.
        target: std::ffi::OsString,

        #[arg(short, long, default_value = ".")]
        /// New location for the file instead of the older
        location: std::ffi::OsString,
    },

    #[command()]
    /// List the files that can be restored.
    ListKidnapped,

    #[command()]
    /// Deletes the contents of the recovery folder permanently.
    Massacre,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Massacre => println!("Nuke!"),
        Command::Kidnap {
            targets: path,
            kill: destroy,
            spare: ignore,
            verbose
        } => println!("{:?} {} {:?} {}", path, destroy, ignore, verbose),
        Command::Free { target, location } => println!("{:?} {:?}", target, location),
        Command::ListKidnapped => println!("restorables"),
    };
}
