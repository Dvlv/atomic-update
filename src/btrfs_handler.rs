use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process;
use std::process::{Command, Stdio};

fn run_command_and_stream_out(
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


fn run_command_and_stream_err(
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

fn run_command(
    cmd_to_run: std::string::String,
    args_for_cmd: Option<&[&str]>,
) -> std::result::Result<std::process::Output, std::io::Error> {
    let mut cmd = Command::new(cmd_to_run);
    if let Some(a) = args_for_cmd {
        cmd.args(a);
    }

    cmd.output()
}

fn get_command_output(
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

pub fn is_root_user() -> bool {
    let uid = get_command_output(String::from("id"), Some(&*vec!["-u"]));
    let is_root = if uid.trim() == "0" {
        true
    } else {
        false
    };
}

pub fn create_snapshots_dir() {
    let is_root = is_root_user();
    if !is_root {
        eprintln!("must be run as root!");
        process::exit(1)
    }

    let snapshots_dir = std::path::Path::new("/.snapshots/");

    if !snapshots_dir.exists() {
        println!("Creating Snapshots Directory: {:?}", snapshots_dir);
        run_command(String::from("mkdir"), Some(&*vec![snapshots_dir.to_str().unwrap()]));
    }
}


