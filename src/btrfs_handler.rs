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

    let snapshots_dir = Path::new("/.au-snapshots/");

    make_dir_if_not_exists(snapshots_dir);
}

pub fn create_root_snapshot(snapshot_target_dir: &Path) -> std::io::Result<()> {
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

pub fn run_command_in_snapshot_chroot(snapshot_target_dir: &Path, command: String, args: Option<&[&str]>) -> std::io::Result<()> {
    // Chroots dont have /etc/resolv.conf, so network doesnt work
    // copy from host into snapshot
    let resolv_loc = format!("{}/etc/resolv.conf", snapshot_target_dir.to_str().unwrap());
    run_command(String::from("rm"), Some(vec![resolv_loc.as_str()].as_slice()));
    run_command(String::from("cp"), Some(vec!["/etc/resolv.conf", resolv_loc.as_str()].as_slice()));

    let mut chroot_plus_command = vec![snapshot_target_dir.to_str().unwrap(), command.as_str()];
    if let Some(a) = args {
        for s in a.iter() {
            chroot_plus_command.push(*s);
        }
    };
    run_command(String::from("chroot"), Some(chroot_plus_command.as_slice()));

    Ok(())
}


pub fn get_next_snapshot_path() -> Result<String, std::io::Error> {
    let snapshots_path = Path::new("/.au-snapshots");
    if !snapshots_path.is_dir() {
        create_snapshots_dir();
        return Ok(String::from("/.au-snapshots/1"));
    }
    let mut entries = std::fs::read_dir(snapshots_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    entries.sort();
    println!("{:?}", entries);

    if entries.len() < 1 {
        return Ok(String::from("/.au-snapshots/1"));
    }

    for entry in entries.iter().rev() {
        let entry_str = entry.to_str().unwrap();
        let entry_folder = entry_str.split("/").last().unwrap();
        if let Ok(num) = entry_folder.parse::<i32>() {
            return Ok(String::from(format!("/.au-snapshots/{}", num + 1)));
        }
    }

    let er = std::io::Error::new(std::io::ErrorKind::Other, "Could not parse the au-snapshots directory");
    Err(er)
}
