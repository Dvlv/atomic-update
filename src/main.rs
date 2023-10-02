use std::{env, process};

use btrfs_handler::*;

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
