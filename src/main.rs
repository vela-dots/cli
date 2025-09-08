use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use which::which;

#[derive(Parser)]
#[command(name="vela", version, about="Vela CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Command
}

#[derive(Subcommand)]
enum Command {
    /// Environment checks for shell prerequisites
    Doctor,
    /// Reload the running shell (placeholder)
    ShellReload,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Doctor => doctor(),
        Command::ShellReload => shell_reload(),
    }
}

fn doctor() -> Result<()> {
    let tools = ["hyprctl", "qtpaths", "qmake"];
    let mut missing = vec![];
    for t in tools {
        if which(t).is_err() { missing.push(t); }
    }
    if missing.is_empty() {
        println!("All good ✅");
        Ok(())
    } else {
        Err(anyhow!("Missing tools: {}", missing.join(", ")))
    }
}

fn shell_reload() -> Result<()> {
    // Placeholder: replace with proper IPC or signal later
    println!("Shell reload requested (TODO: implement IPC)");
    Ok(())
}
