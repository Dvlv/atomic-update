use std::{env, io, process};
use std::path::Path;
use std::process::exit;

use btrfs_handler::*;

use crate::config_handler::{create_config_file, read_config_file};
use crate::utils::get_root_partition_device;

mod btrfs_handler;
mod config_handler;
mod utils;

fn usage() {
    println!("Usage:");
    println!("au init - Initialise a system with atomic-update.");
    println!("au update - Update your system in a new snapshot.");
    println!("au exec [command arg1 arg2] - Run a command in a new snapshot. e.g. atomic-update exec dnf install sshfs -y");
    println!("au install - Install a package into a new snapshot");
    println!("au rollback - Undo last operation.");
}

fn init() {
    if !is_root_user() {
        eprintln!("init must be run as root!");
        process::exit(1)
    }

    println!("atomic-update is alpha software and is not yet suitable for important systems. If you do not wish to risk this, please enter Ctrl+C now.\n\nI acknowledge that using atomic-update could possibly corrupt my system, and use it at my own risk.\n Please enter [y/yes]");
    let mut yes_input = String::new();
    while true {
        match io::stdin().read_line(&mut yes_input) {
            Ok(_n) => {
                yes_input = yes_input.trim().to_string();
                if yes_input.to_lowercase() == "y" || yes_input.to_lowercase() == "yes" {
                    break;
                } else {
                    println!("Please enter 'y' or 'yes', not {}", yes_input);
                    yes_input = String::from("");
                }
            }
            Err(_error) => eprintln!("An unexpected error occurred"),
        }
    }
    create_snapshots_dir();

    create_config_file();
}

fn update() {
    let next_snapshot_location = get_next_snapshot_path().expect("Could not parse snapshot dir");
    let next_snapshot_path = Path::new(next_snapshot_location.as_str());
    create_root_snapshot(next_snapshot_path).expect("Could not create snapshot");
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

fn exec_cmd(cmd_args: &mut Vec<String>) {
    let next_snapshot_location = get_next_snapshot_path().expect("Could not parse snapshot dir");
    let next_snapshot_path = Path::new(next_snapshot_location.as_str());
    create_root_snapshot(next_snapshot_path).expect("Could not create snapshot");

    let cmd_to_run = cmd_args[0].clone();

    if cmd_args.len() == 1 {
        match run_command_in_snapshot_chroot(next_snapshot_path, cmd_to_run.clone(), None) {
            Ok(()) => {
                println!("Worked!");
                swap_snapshot_to_root(next_snapshot_path);
            }
            Err(e) => {
                println!("nope, {:?}", e);
            }
        }
    } else {
        let args_to_run: Vec<&str> = cmd_args[1..].iter().map(|s| s.as_str()).collect();

        match run_command_in_snapshot_chroot(next_snapshot_path, cmd_to_run.clone(), Some(&args_to_run)) {
            Ok(()) => {
                println!("Worked!");
                swap_snapshot_to_root(next_snapshot_path);
            }
            Err(e) => {
                println!("nope, {:?}", e);
            }
        }
    }
}

fn rollback() {
    println!("Not implemented yet");
}

fn deb() {
    println!("{}", get_root_partition_device());
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
            if args.len() < 3 {
                println!("Not enough args passed to exec! \n");
                return usage();
            }
            let mut cmd_args = args[2..args.len()].to_vec();
            exec_cmd(&mut cmd_args);
        }
        "install" => {
            println!("init")
        }
        "rollback" => {
            return rollback();
        }
        "deb" => {
            return deb();
        }
        _ => {
            return usage();
        }
    }
}
