use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;
use std::process::{exit, Command, Stdio};

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
        .map_while(Result::ok)
        .filter(|line| line.contains("usb"))
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
        .map_while(Result::ok)
        .filter(|line| line.contains("usb"))
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
            if !o.stdout.is_empty() {
                result = result
                    + String::from_utf8_lossy(&o.stdout).into_owned().as_ref()
                    + &String::from("\n");
            }

            if !o.stderr.is_empty() {
                result = result
                    + String::from_utf8_lossy(&o.stderr).into_owned().as_ref()
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
        fs::create_dir_all(path)
            .expect(&format!("Could not create {:?} directory!", path.to_str()));
    }
}

pub fn try_detect_distro() -> String {
    if !is_root_user() {
        eprintln!("Must be run as root!");
        exit(1);
    }

    let os_release_content =
        get_command_output(String::from("cat"), Some(&*vec!["/etc/os-release"]));
    let mut os_name = "unknown";
    for line in os_release_content.split('\n') {
        if line.starts_with("NAME=\"") {
            let mut line_chars = line.chars();
            line_chars.next_back();

            os_name = &line_chars.as_str()[6..];
        }
    }

    os_name
        .to_lowercase()
        .replace("linux", "")
        .trim()
        .split(' ')
        .next()
        .unwrap()
        .to_string()
}

pub fn get_root_partition_device() -> String {
    //df --output=source,fstype,target

    let df_args = vec!["--output=source,fstype,target"];

    // TODO use Errors not magic strings!
    let df_out = get_command_output(String::from("df"), Some(df_args.as_slice()));
    if df_out == "fail" {
        return String::from("");
    }

    for line in df_out.split('\n') {
        let line_lower = line.to_ascii_lowercase();
        let df_out_parts = line_lower
            .split(' ')
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>();
        if df_out_parts.len() < 3 {
            continue;
        }
        if df_out_parts[0] == "filesystem" {
            // heading
            continue;
        }

        let ftype = df_out_parts[1];
        let mounted = df_out_parts[2];

        if ftype == "btrfs" && mounted == "/" {
            return String::from(df_out_parts[0]);
        }
    }

    return String::from("");
}
