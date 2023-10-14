use std::fs;
use std::path::Path;
use std::process;
use std::process::exit;

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
    run_command_and_stream_err(String::from("rm"), vec![resolv_loc.as_str()].as_slice());
    run_command_and_stream_err(String::from("cp"), vec!["/etc/resolv.conf", resolv_loc.as_str()].as_slice());

    let mut chroot_plus_command = vec![snapshot_target_dir.to_str().unwrap(), command.as_str()];
    if let Some(a) = args {
        for s in a.iter() {
            chroot_plus_command.push(*s);
        }
    };
    run_command_and_stream_err(String::from("chroot"), chroot_plus_command.as_slice());

    Ok(())
}

pub fn swap_snapshot_to_root(snapshot_path: &Path) {
    let root_subvol_name = get_root_subvolume_name().expect("Could not determine root subvolume name - expecting 'root' or '@'");
    let root_partition_device = get_root_partition_device();
    if root_partition_device.as_str() == "" {
        eprintln!("Failed to detect root partition device, please set it manually in /etc/atomic-update.conf");
        exit(1);
    }

    let root_subvol_path = format!("/mnt/{}", root_subvol_name);
    let root_subvol_path = Path::new(root_subvol_path.as_str());

    let rollback_subvol_path = Path::new("/mnt/rollback");

    let new_path_to_new_root = format!("/mnt/rollback{}", snapshot_path.to_str().unwrap());
    let new_path_to_new_root = Path::new(new_path_to_new_root.as_str());

    let new_rollback_path = format!("/mnt/{}/.au-snapshots/rollback", root_subvol_name);
    let new_rollback_path = Path::new(new_rollback_path.as_str());

    let mount_command = format!("mount -t btrfs -o subvolid=5 {} /mnt", root_partition_device);
    let mount_command_parts = mount_command.split(" ").collect::<Vec<_>>();

    println!("Swapping {} to new root, moving current root to /.au-snapshots/rollback", snapshot_path.to_str().unwrap());

    let was_mounted = run_command(mount_command_parts[0].to_string(), Some(mount_command_parts[1..].iter().as_slice()));

    if let Err(e) = was_mounted {
        eprintln!("Failed mounting {} to /mnt", root_partition_device);
        exit(1);
    }

    fs::rename(root_subvol_path, rollback_subvol_path).expect("Failed to move subvolume at step 1");  // mv /mnt/root /mnt/rollback
    fs::rename(new_path_to_new_root, root_subvol_path).expect("Filed to move subvolume at step 2");  // mv /mnt/rollback/.au-snapshots/1 /mnt/root
    fs::rename(rollback_subvol_path, new_rollback_path).expect("Failed to move subvolume at step 3");  // mv /mnt/rollback /mnt/root/.au-snapshots/rollback

    let was_unmounted = run_command(String::from("umount"), Some(vec!["/mnt"].as_slice()));
    if let Err(e) = was_unmounted {
        eprintln!("Failed unmounting /mnt, please do this manually");
    }
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
            let next_dir = format!("/.au-snapshots/{}", num + 1);
            if !Path::new(&next_dir).exists() {
                return Ok(next_dir);
            }
        }
    }

    return Ok(String::from("/.au-snapshots/1"));
}
