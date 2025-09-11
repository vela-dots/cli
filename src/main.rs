// Migrated from cli-rs/src/main.rs
mod util;
mod hypr;
mod paths;
mod notify;
mod scheme;
mod wallpaper;
mod theme;
mod cmds;
mod languages;

use anyhow::Result;
use clap::{Parser, Subcommand, ArgAction, CommandFactory};

#[derive(Parser, Debug)]
#[command(name = "vela", disable_version_flag = true, about = "Main control tool for Vela dotfiles")]
struct Cli {
    /// Print extended version info
    #[arg(short = 'v', long = "version", action = ArgAction::SetTrue)]
    version: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Shell(cmds::ShellArgs),
    Toggle { workspace: String },
    Scheme { #[command(subcommand)] cmd: cmds::SchemeCmd },
    Screenshot(cmds::ScreenshotArgs),
    Record(cmds::RecordArgs),
    Clipboard(cmds::ClipboardArgs),
    Emoji(cmds::EmojiArgs),
    Wallpaper(cmds::WallpaperArgs),
    Resizer(cmds::ResizerArgs),
    /// Editor integrations (VSCodium)
    Editor(cmds::EditorArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.version { return cmds::cmd_version(); }
    match cli.command {
        Some(Commands::Shell(a)) => cmds::cmd_shell(a),
        Some(Commands::Toggle { workspace }) => cmds::cmd_toggle(workspace),
        Some(Commands::Scheme { cmd: cmds::SchemeCmd::List { names, flavors, modes, variants } }) => scheme::scheme_list(names, flavors, modes, variants),
        Some(Commands::Scheme { cmd: cmds::SchemeCmd::Get { name, flavor, mode, variant } }) => scheme::scheme_get(name, flavor, mode, variant),
        Some(Commands::Scheme { cmd: cmds::SchemeCmd::Set(a) }) => scheme::scheme_set(a),
        Some(Commands::Screenshot(a)) => cmds::cmd_screenshot(a),
        Some(Commands::Record(a)) => cmds::cmd_record(a),
        Some(Commands::Clipboard(a)) => cmds::cmd_clipboard(a),
        Some(Commands::Emoji(a)) => cmds::cmd_emoji(a),
        Some(Commands::Wallpaper(a)) => cmds::cmd_wallpaper(a),
        Some(Commands::Resizer(a)) => cmds::cmd_resizer(a),
        Some(Commands::Editor(a)) => cmds::cmd_editor(a),
        None => { let _ = Cli::command().print_help(); println!(); Ok(()) }
    }
}
