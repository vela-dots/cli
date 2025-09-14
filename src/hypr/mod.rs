use anyhow::{Context, Result};
use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::vec;

pub fn qs_run(args: &[&str]) -> Result<String> {
    let output = Command::new("qs")
        .args(args)
        .output()
        .with_context(|| format!("failed to run qs with arguments {:?}", args))?;
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "qs {:?} failed with status of {}",
            args,
            output.status
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn launch_shell(daemon: bool, log_rules: Option<&str>) -> Result<()> {
    let mut args = vec!["-c", "vela", "-n"];
    if let Some(rules) = log_rules {
        args.extend(["--log-rules", rules]);
    }
    if daemon {
        args.push("-d");
        Command::new("qs")
            .args(&args)
            .spawn()
            .with_context(|| "failed to launch the shell")?
            .wait()?;
    } else {
        let mut child = Command::new("qs")
            .args(&args)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "failed to launch the shell")?;
        let stdout = child.stdout.take().expect("child stdout");
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line?;

            println!("{}", line);
        }
        child.wait()?;
    }
    Ok(())
}

pub fn kill_shell() -> Result<String> {
    qs_run(&["-c", "vela", "kill"])
}

pub fn show_ipc() -> Result<String> {
    qs_run(&["-c", "vela", "ipc", "show"])
}

pub fn log_shell(log_rules: Option<&str>) -> Result<String> {
    let mut args = vec!["-c", "vela", "log"];
    if let Some(rules) = log_rules {
        args.extend(["-r", rules]);
    }
    qs_run(&args)
}

pub fn ipc_call(message: &[String]) -> Result<String> {
    let mut args: Vec<&str> = vec!["-c", "vela", "ipc", "call"];
    for s in message {
        args.push(s);
    }
    qs_run(&args)
}
