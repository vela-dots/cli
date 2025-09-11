use anyhow::Result;
use std::process::{Command, Stdio};

pub fn notify(args: &[&str]) -> Result<String> {
    let out = Command::new("notify-send")
        .arg("-a").arg("vela-cli")
        .args(args)
        .stdout(Stdio::piped())
        .output()?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn close_notification(id: &str) {
    let _ = Command::new("gdbus").args([
        "call","--session","--dest=org.freedesktop.Notifications","--object-path=/org/freedesktop/Notifications",
        "--method=org.freedesktop.Notifications.CloseNotification", id
    ]).stdout(Stdio::null()).status();
}

