use std::path::Path;
use std::process;
use std::os::unix::fs::chroot;

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

pub fn create_root_snapshot(snapshot_target_dir: &Path) -> std::io::Result<()>{
    let success = run_command(String::from("btrfs"), Some(&*vec!["subvolume", "snapshot", "/", snapshot_target_dir.to_str().unwrap()]));

    match success {
        Ok(output) => {
            println!("Snapshot created at {:?}", snapshot_target_dir.as_os_str());
            Ok(())

        }
        Err(error) => {
            eprintln!("Error creating snapshot: {:?}", error);
            Err(error)
        }
    }
}

pub fn run_command_in_snapshot_chroot(snapshot_target_dir: &Path, command: String, args: Option<&[&str]>) -> std::io::Result<()>{
    chroot(snapshot_target_dir)?;
    std::env::set_current_dir("/")?;  //TODO confirm
    println!("Executing: {} {:?}", command, args);
    run_command(command, args)?;

    Ok(())
}



pub fn get_next_snapshot_path() -> String {
    return String::from("/.snapshots/1");

}
