use anyhow::Result;
use clap::{Parser, Subcommand, Args};

mod cmds;
mod hypr;
mod util;

#[derive(Parser, Debug)]
#[command(
    name = "vela",
    about = "Main control tool for Vela dotfiles",
    disable_version_flag = true,
)]

struct Cli {
    #[arg(short = 'v', long = "version")]
    version: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Shell(ShellArgs),    
    Toggle(ToggleArgs),    
    Scheme(SchemeArgs),    
    Screenshot(ScreenshotArgs),    
    Record(RecordArgs),    
    Clipboard(ClipboardArgs),    
    Emoji(EmojiArgs),    
    Wallpaper(WallpaperArgs),    
    Resizer(ResizerArgs),    
    Editor(EditorArgs),    
    Install(InstallArgs),
    Version(Version),    
}

#[derive(Args, Debug)]
struct ShellArgs {
    #[arg(short, long)]
    daemon: bool,
}

#[derive(Args, Debug)]
struct ToggleArgs {
    workspace: String,
}

#[derive(Args, Debug)]
struct SchemeArgs {
    #[arg()]
    action: Vec<String>,
}

#[derive(Args, Debug)]
struct ScreenshotArgs {}

#[derive(Args, Debug)]
struct RecordArgs {}

#[derive(Args, Debug)]
struct ClipboardArgs {}

#[derive(Args, Debug)]
struct EmojiArgs {}

#[derive(Args, Debug)]
struct WallpaperArgs {}

#[derive(Args, Debug)]
struct ResizerArgs {}

#[derive(Args, Debug)]
struct EditorArgs {}

#[derive(Args, Debug)]
struct InstallArgs {}

#[derive(Args, Debug)]
struct Version {}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Shell(args))             => cmds::run_shell(args),
        Some(Commands::Toggle(args))            => cmds::run_toggle(args),
        Some(Commands::Scheme(args))            => cmds::run_scheme(args),
        Some(Commands::Screenshot(args))        => cmds::run_screenshot(args),
        Some(Commands::Record(args))            => cmds::run_record(args),
        Some(Commands::Clipboard(args))         => cmds::run_clipboard(args),
        Some(Commands::Emoji(args))             => cmds::run_emoji(args),
        Some(Commands::Wallpaper(args))         => cmds::run_wallpaper(args),
        Some(Commands::Resizer(args))           => cmds::run_resizer(args),
        Some(Commands::Editor(args))            => cmds::run_editor(args),
        Some(Commands::Install(args))           => cmds::run_install(args),
        Some(Commands::Version(cmd))           => cmds::run_version(cmd),
        None => {
            Ok(())
        }
    }
}