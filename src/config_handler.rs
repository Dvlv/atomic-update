use std::fs;
use std::fs::{File, OpenOptions, read_to_string};
use std::io::Write;
use std::path::Path;

use crate::btrfs_handler::get_root_subvolume_name;
use crate::utils::*;

pub struct ConfigOpts {
    pub(crate) package_manager: String,
    pub(crate) update_command: String,
    pub(crate) install_command: String,
    pub(crate) yes_flag: String,
    pub(crate) root_partition: String,
    pub(crate) root_subvolume: String,
}

fn populate_config_file_with_defaults() {
    let mut package_manager = "";
    let mut update_command = "";
    let mut install_command = "";
    let mut yes_flag = "";

    let distro = try_detect_distro();
    match distro.as_ref() {
        "mint" | "ubuntu" | "debian" => {
            println!("Detected {}, assuming your package manager is apt", distro);
            package_manager = "apt";
            update_command = "upgrade";
            install_command = "install";
            yes_flag = "-y";
        }
        "suse" | "opensuse" => {
            println!("Detected {}, assuming your package manager is zypper", distro);
            package_manager = "zypper";
            update_command = "update";
            install_command = "install";
            yes_flag = "-y";
        }
        "fedora" | "centos" | "oracle" => {
            println!("Detected {}, assuming your package manager is dnf", distro);
            package_manager = "dnf";
            update_command = "update";
            install_command = "install";
            yes_flag = "-y";
        }
        "arch" | "endeavouros" => {
            println!("Detected {}, assuming your package manager is pacman", distro);
            package_manager = "pacman";
            update_command = "-Syu";
            install_command = "-S";
            yes_flag = "--noconfirm";
        }
        "unknown" => {
            println!("Your distro could not be detected. Please populate /etc/atomic-update.conf manually");
            package_manager = "please-replace";
            update_command = "please-replace";
            install_command = "please-replace";
            yes_flag = "please-replace";
        }
        _ => {
            println!("An Unknown Error has occurred.");
        }
    }

    let root_subvol = get_root_subvolume_name();
    let root_partition = get_root_partition_device();

    if package_manager.len() > 0 {
        let config_contents = format!("PACKAGE_MANAGER {}\nUPDATE_COMMAND {}\nINSTALL_COMMAND {}\nYES_FLAG {}\n", package_manager, update_command, install_command, yes_flag);
        fs::write("/etc/atomic-update.conf", config_contents).expect("Unable to write to /etc/atomic-update.conf");
    }

    if let Some(subvol) = root_subvol {
        let mut cfg_file = OpenOptions::new().append(true).open("/etc/atomic-update.conf").unwrap();
        writeln!(cfg_file, "ROOT_SUBVOLUME {}", subvol.trim()).expect("Failed to write to config file");
    }

    // TODO this should be a Result/Option in the future
    if !root_partition.is_empty() {
        let mut cfg_file = OpenOptions::new().append(true).open("/etc/atomic-update.conf").unwrap();
        writeln!(cfg_file, "ROOT_PARTITION {}", root_partition.trim()).expect("Failed to write to config file");
    }
}

pub fn create_config_file() {
    let config_file_path = Path::new("/etc/atomic-update.conf");

    if !config_file_path.exists() {
        File::create("/etc/atomic-update.conf").expect("Could not create /etc/atomic-update.conf");
        populate_config_file_with_defaults();
    }
}

pub fn read_config_file() -> Result<ConfigOpts, std::io::Error> {
    let config_file_path = Path::new("/etc/atomic-update.conf");

    if !config_file_path.exists() {
        let err = std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find atomic update config file!");
        return Err(err);
    }

    let mut update_command = "update";
    let mut package_manager = "dnf";
    let mut install_command = "install";
    let mut yes_flag = "-y";
    let mut root_partition = "";
    let mut root_subvolume = "";

    // must be a more elegant way to do this
    let file_contents = read_to_string(config_file_path).unwrap();
    for line in file_contents.lines() {
        if line.starts_with("UPDATE_COMMAND") {
            update_command = line.split(" ").last().unwrap();
        } else if line.starts_with("PACKAGE_MANAGER") {
            package_manager = line.split(" ").last().unwrap();
        } else if line.starts_with("INSTALL_COMMAND") {
            install_command = line.split(" ").last().unwrap();
        } else if line.starts_with("YES_FLAG") {
            yes_flag = line.split(" ").last().unwrap();
        } else if line.starts_with("ROOT_PARTITION") {
            root_partition = line.split(" ").last().unwrap();
        } else if line.starts_with("ROOT_SUBVOLUME") {
            root_subvolume = line.split(" ").last().unwrap();
        }
    }

    let co = ConfigOpts {
        update_command: update_command.to_string(),
        package_manager: package_manager.to_string(),
        install_command: install_command.to_string(),
        yes_flag: yes_flag.to_string(),
        root_partition: root_partition.to_string(),
        root_subvolume: root_subvolume.to_string(),
    };

    Ok(co)
}