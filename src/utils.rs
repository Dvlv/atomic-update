use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;
use std::process::{Command, exit, Stdio};

use crate::btrfs_handler::is_root_user;

pub fn run_command_and_stream_out(
    cmd_to_run: std::string::String,
    args_for_cmd: &[&str],
) -> Result<(), Error> {
    let stdout = Command::new(cmd_to_run)
        .args(args_for_cmd)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));

    Ok(())
}


pub fn run_command_and_stream_err(
    cmd_to_run: std::string::String,
    args_for_cmd: &[&str],
) -> Result<(), Error> {
    let stdout = Command::new(cmd_to_run)
        .args(args_for_cmd)
        .env("GIT_EXTERNAL_DIFF", "difft")
        .stderr(Stdio::piped())
        .spawn()?
        .stderr
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));

    Ok(())
}

pub fn run_command(
    cmd_to_run: std::string::String,
    args_for_cmd: Option<&[&str]>,
) -> std::result::Result<std::process::Output, std::io::Error> {
    let mut cmd = Command::new(cmd_to_run);
    if let Some(a) = args_for_cmd {
        cmd.args(a);
    }

    cmd.output()
}

pub fn get_command_output(
    cmd_to_run: std::string::String,
    args_for_cmd: Option<&[&str]>,
) -> std::string::String {
    let output = run_command(cmd_to_run, args_for_cmd);

    match output {
        Ok(o) => {
            let mut result = String::from("");
            if &o.stdout.len() > &0 {
                result = result
                    + &String::from_utf8_lossy(&o.stdout).to_owned().to_string()
                    + &String::from("\n");
            }

            if &o.stderr.len() > &0 {
                result = result
                    + &String::from_utf8_lossy(&o.stderr).to_owned().to_string()
                    + &String::from("\n");
            }

            result
        }
        Err(_) => "fail".to_string(),
    }
}

pub fn make_dir_if_not_exists(path: &Path) {
    if !path.exists() {
        println!("Creating Snapshots Directory: {:?}", path);
        fs::create_dir_all(path).expect(&*format!("Could not create {:?} directory!", path.to_str()));
    }
}

pub fn try_detect_distro() -> String {
    if !is_root_user() {
        eprintln!("Must be run as root!");
        exit(1);
    }

    let os_release_content = get_command_output(String::from("cat"), Some(&*vec!["/etc/os-release"]));
    let mut os_name = "unknown";
    for line in os_release_content.split("\n") {
        if line.starts_with("NAME=\"") {
            let mut line_chars = line.chars();
            line_chars.next_back();

            os_name = &line_chars.as_str()[6..];
        }
    }

    os_name.to_lowercase().split(" ").next().unwrap().to_string()
}