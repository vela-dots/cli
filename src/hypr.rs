use anyhow::{anyhow, Result};
use serde_json::Value;
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};
use std::env;

fn socket_base() -> Result<String> {
    let xdg = env::var("XDG_RUNTIME_DIR").map_err(|_| anyhow!("XDG_RUNTIME_DIR not set"))?;
    let sig = env::var("HYPRLAND_INSTANCE_SIGNATURE").map_err(|_| anyhow!("HYPRLAND_INSTANCE_SIGNATURE not set"))?;
    Ok(format!("{}/hypr/{}", xdg, sig))
}

pub fn socket_path() -> Result<String> { Ok(format!("{}/.socket.sock", socket_base()?)) }
pub fn socket2_path() -> Result<String> { Ok(format!("{}/.socket2.sock", socket_base()?)) }

pub fn message(msg: &str, json: bool) -> Result<Value> {
    let path = socket_path()?;
    let mut stream = UnixStream::connect(path)?;
    let final_msg = if json { format!("j/{}", msg) } else { msg.to_string() };
    stream.write_all(final_msg.as_bytes())?;

    let mut buf = Vec::new();
    let mut tmp = [0u8; 8192];
    loop { let n = stream.read(&mut tmp)?; if n == 0 { break; } buf.extend_from_slice(&tmp[..n]); }
    let s = String::from_utf8_lossy(&buf).to_string();
    if json { Ok(serde_json::from_str(&s)?) } else { Ok(Value::String(s)) }
}

pub fn dispatch(dispatcher: &str, args: &[&str]) -> Result<bool> {
    let msg = format!("dispatch {} {}", dispatcher, args.join(" ")).trim().to_string();
    let v = message(&msg, false)?;
    Ok(matches!(v, Value::String(ref s) if s == "ok"))
}

pub fn batch(msgs: &[&str], json: bool) -> Result<Value> {
    let mut payloads: Vec<String> = Vec::new();
    for m in msgs { let mut s = m.to_string(); if json { s = format!("j/{}", s); } payloads.push(s); }
    let final_msg = format!("[[BATCH]]{}", payloads.join(";"));
    message(&final_msg, false)
}

