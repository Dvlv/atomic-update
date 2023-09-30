use std::path::Path;
use std::process;

use crate::utils::*;

pub fn is_root_user() -> bool {
    let uid = get_command_output(String::from("id"), Some(&*vec!["-u"]));
    let is_root = if uid.trim() == "0" {
        true
    } else {
        false
    };

    is_root
}

pub fn get_root_subvolume_name() -> Option<String> {
    let subvols = get_command_output(String::from("btrfs"), Some(&*vec!["subvolume", "list", "/"]));

    let subvol_output_lines = subvols.split("\n");

    // root = Fedora
    // @ = Opensuse, Mint
    let probable_root_names = vec!["root", "@"];

    for line in subvol_output_lines {
        let subvol_path = line.split(" ").last().unwrap();
        if probable_root_names.contains(&subvol_path) {
            return Some(subvol_path.to_string());
        }
    }

    None
}

pub fn create_snapshots_dir() {
    let is_root = is_root_user();
    if !is_root {
        eprintln!("must be run as root!");
        process::exit(1)
    }

    let snapshots_dir = Path::new("/.snapshots/");

    make_dir_if_not_exists(snapshots_dir);
}

pub fn create_root_snapshot(snapshot_target_dir: &Path) {
    let success = run_command(String::from("btrfs"), Some(&*vec!["subvolume", "snapshot", "/", snapshot_target_dir.as_os_str()]);

    match success {
        Ok(Output) => {println!("Snapshot created at {:?}", snapshot_target_dir.as_os_str())},
        Err(error) => {eprintln!("Error creating snapshot: {:?}", error)}
    }
}

