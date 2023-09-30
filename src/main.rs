use std::{env, process};

use btrfs_handler::*;

mod btrfs_handler;

fn usage() {
    println!("Usage:");
    println!("au init - Initialise a system with atomic-update");
    println!("au update - Update your system in a new snapshot");
    println!("au exec - Run a command in a new snapshot");
    println!("au install - Install a package into a new snapshot");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return usage();
    }

    match args[1].as_str() {
        "init" => {
            if !is_root_user() {
                eprintln!("init must be run as root!");
                process::exit(1)
            }
            create_snapshots_dir();
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