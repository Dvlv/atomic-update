use std::fs;
use std::fs::File;
use std::path::Path;

use crate::utils::*;

fn populate_config_file_with_defaults() {
    let mut package_manager = "";
    let mut update_command = "";
    let mut install_command = "";

    let distro = try_detect_distro();
    match distro.as_ref() {
        "mint" | "ubuntu" | "debian" => {
            println!("Detected {}, assuming your package manager is apt", distro);
            package_manager = "apt";
            update_command = "upgrade";
            install_command = "install";
        }
        "suse" | "opensuse" => {
            println!("Detected {}, assuming your package manager is zypper", distro);
            package_manager = "zypper";
            update_command = "update";
            install_command = "install";
        }
        "fedora" | "centos" | "oracle" => {
            println!("Detected {}, assuming your package manager is dnf", distro);
            package_manager = "dnf";
            update_command = "update";
            install_command = "install";
        }
        "arch" | "endeavour" => {
            println!("Detected {}, assuming your package manager is pacman", distro);
            package_manager = "pacman";
            update_command = "-Syu";
            install_command = "-S";
        }
        "unknown" => {
            println!("Your distro could not be detected. Please populate /etc/atomic-update.conf manually");
            package_manager = "please-replace";
            update_command = "please-replace";
            install_command = "please-replace";
        }
        _ => {
            println!("An Unknown Error has occurred.");
        }
    }

    if package_manager.len() {
        let config_contents = format!("PACKAGE_MANAGER {}\nUPDATE_COMMAND {}\nINSTALL_COMMAND {}", package_manager, update_command, install_command);
        fs::write("/etc/atomic-update.conf", config_contents).expect("Unable to write to /etc/atomic-update.conf");
    }
}

pub fn create_config_file() {
    let config_file_path = Path::new("/etc/atomic-update.conf");

    if !config_file_path.exists() {
        File::create("/etc/atomic-update.conf").expect("Could not create /etc/atomic-update.conf");
        populate_config_file_with_defaults();
    }
}

