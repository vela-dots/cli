use anyhow::Result;
use crate::Version;

pub fn run(_cmd: Version) -> Result<()> {
    // TODO: Add version utilities
    println!("Version command has been run.");
    Ok(())
}