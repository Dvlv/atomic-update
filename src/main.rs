use std::{env, process};
use std::path::Path;
use std::process::exit;

use btrfs_handler::*;
use crate::config_handler::read_config_file;

use crate::utils::try_detect_distro;

mod btrfs_handler;
mod config_handler;
mod utils;

fn usage() {
    println!("Usage:");
    println!("au init - Initialise a system with atomic-update");
    println!("au update - Update your system in a new snapshot");
    println!("au exec - Run a command in a new snapshot");
    println!("au install - Install a package into a new snapshot");
}

fn init() {
    if !is_root_user() {
        eprintln!("init must be run as root!");
        process::exit(1)
    }
    // let root_subvol = get_root_subvolume_name();
    // if let Some(rs) = root_subvol {
    //     println!("{}", rs);
    // } else {
    //     eprintln!("Could not locate a root subvolume, please set manually in the config!");
    //     process::exit(1)
    // }

    //create_snapshots_dir();
    //create_config_file();
    println!("{}", try_detect_distro());
}

fn update() {
    let next_snapshot_location = get_next_snapshot_path();
    let next_snapshot_path = Path::new(next_snapshot_location.as_str());
    create_root_snapshot(Path::new("/.snapshots/path1")).expect("Could not create snapshot");
    // TODO config
    let mut package_manager = String::from("");
    let mut update_command = String::from("");
    if let Ok(opts) = read_config_file() {
        package_manager = opts.package_manager;
        update_command = opts.update_command;
    }

    if package_manager.is_empty() || update_command.is_empty() {
        eprintln!("Config could not be read, please edit /etc/atomic-update.conf!");
        exit(1);
    }

    run_command_in_snapshot_chroot(next_snapshot_path, package_manager, Some(vec![update_command.as_str()].as_slice()));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return usage();
    }

    match args[1].as_str() {
        "init" => {
            init()
        }
        "update" => {
            println!("init")
        }
        "exec" => {
            println!("init")
        }
        "install" => {
            println!("init")
        }
        _ => {
            return usage();
        }
    }
}
