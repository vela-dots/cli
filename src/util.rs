use anyhow::{anyhow, Result};
use sha2::Digest;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn run(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd).args(args).status()?;
    if !status.success() { return Err(anyhow!(format!("command failed: {} {:?}", cmd, args))); }
    Ok(())
}

pub fn run_output(cmd: &str, args: &[&str]) -> Result<String> {
    let out = Command::new(cmd).args(args).stdout(Stdio::piped()).output()?;
    if !out.status.success() { return Err(anyhow!(format!("command failed: {} {:?}", cmd, args))); }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

pub fn hash_file(path: &Path) -> Result<String> {
    let mut f = fs::File::open(path)?;
    let mut hasher = sha2::Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = f.read(&mut buf)?; if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

pub fn is_valid_image(p: &Path) -> bool {
    matches!(p.extension().and_then(|s| s.to_str()), Some("jpg"|"jpeg"|"png"|"webp"|"tif"|"tiff"))
}

