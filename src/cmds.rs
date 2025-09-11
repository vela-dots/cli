use anyhow::{anyhow, Result};
use clap::{Args, ArgAction, Subcommand};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::Write;
use rand::seq::SliceRandom;
use time::OffsetDateTime;

use crate::notify::notify;
use crate::paths::{VPATHS, ensure_parent};
use crate::scheme;
use crate::util::{run_output, run};
use crate::wallpaper;
use crate::languages;

#[derive(Args, Debug)]
pub struct ShellArgs {
    pub message: Vec<String>,
    #[arg(short, long)] pub daemon: bool,
    #[arg(short, long)] pub show: bool,
    #[arg(short, long)] pub log: bool,
    #[arg(short, long)] pub kill: bool,
    #[arg(long = "log-rules")] pub log_rules: Option<String>,
}

#[derive(Args, Debug)]
pub struct ScreenshotArgs {
    #[arg(short, long, num_args=0..=1, default_missing_value="slurp", value_name="GEOM")] pub region: Option<String>,
    #[arg(short, long)] pub freeze: bool,
}

#[derive(Args, Debug)]
pub struct RecordArgs {
    #[arg(short, long, num_args=0..=1, default_missing_value="slurp", value_name="GEOM")] pub region: Option<String>,
    #[arg(short, long)] pub sound: bool,
}

#[derive(Args, Debug)]
pub struct ClipboardArgs { #[arg(short, long)] pub delete: bool }

#[derive(Args, Debug)]
pub struct EmojiArgs { #[arg(short, long)] pub picker: bool, #[arg(short, long)] pub fetch: bool }

#[derive(Subcommand, Debug)]
pub enum SchemeCmd {
  List {
    #[arg(short='n', long="names", action=clap::ArgAction::SetTrue)] names: bool,
    #[arg(short='f', long="flavors", action=clap::ArgAction::SetTrue)] flavors: bool,
    #[arg(short='m', long="modes", action=clap::ArgAction::SetTrue)] modes: bool,
    #[arg(short='v', long="variants", action=clap::ArgAction::SetTrue)] variants: bool,
  },
  Get {
    #[arg(short='n', long="name", action=clap::ArgAction::SetTrue)] name: bool,
    #[arg(short='f', long="flavor", action=clap::ArgAction::SetTrue)] flavor: bool,
    #[arg(short='m', long="mode", action=clap::ArgAction::SetTrue)] mode: bool,
    #[arg(short='v', long="variant", action=clap::ArgAction::SetTrue)] variant: bool,
  },
  Set(scheme::SchemeSetArgs)
}

#[derive(Args, Debug)]
pub struct WallpaperArgs {
    #[arg(short, long, num_args=0..=1, value_name="PATH")] pub print: Option<PathBuf>,
    #[arg(short, long, num_args=0..=1, value_name="DIR")] pub random: Option<PathBuf>,
    #[arg(short, long)] pub file: Option<PathBuf>,
    #[arg(short = 'n', long = "no-filter")] pub no_filter: bool,
    #[arg(short, long, default_value_t = 0.8)] pub threshold: f32,
    #[arg(short = 'N', long = "no-smart")] pub no_smart: bool,
}

#[derive(Args, Debug)]
pub struct ResizerArgs { #[arg(short, long)] pub daemon: bool, pub pattern: Option<String>, pub match_type: Option<String>, pub width: Option<String>, pub height: Option<String>, pub actions: Option<String> }

pub fn cmd_version() -> Result<()> {
    println!("Vela CLI: {}", env!("CARGO_PKG_VERSION"));
    if which::which("qs").is_ok() { let v = run_output("qs", &["--version"]).unwrap_or_default(); if !v.is_empty() { println!("Quickshell: {}", v.trim()); } } else { println!("Quickshell: not in PATH"); }
    if let Ok(root) = std::env::current_dir() { let git_dir = root.join(".git"); if git_dir.exists() { if let Ok(msg) = run_output("git", &["--git-dir", git_dir.to_str().unwrap_or(""), "rev-list", "--format=%B", "--max-count=1", "HEAD"]) { let mut it = msg.lines(); if let Some(first) = it.next() { let sha = first.split_whitespace().nth(1).unwrap_or("?"); println!("Repo last commit: {}", sha); } } } }
    Ok(())
}

pub fn cmd_shell(args: ShellArgs) -> Result<()> {
    let mut base = vec!["-c", "vela", "-n"];
    if let Some(rules) = args.log_rules.as_ref() { base.push("--log-rules"); base.push(rules); }
    if args.show { let out = run_output("qs", &["-c","vela","ipc","show"])?; print!("{}", out); return Ok(()); }
    if args.log { let out = if let Some(r) = args.log_rules.as_ref() { run_output("qs", &["-c","vela","log","-r",r])? } else { run_output("qs", &["-c","vela","log"]) ? }; print!("{}", out); return Ok(()); }
    if args.kill { let _ = run_output("qs", &["-c","vela","kill"])?; return Ok(()); }
    if !args.message.is_empty() { let mut full = vec!["-c","vela","ipc","call"]; full.extend(args.message.iter().map(|s| s.as_str())); let out = run_output("qs", &full)?; print!("{}", out); return Ok(()); }
    if args.daemon { let _ = Command::new("qs").args(&base).arg("-d").status()?; return Ok(()); }
    let mut child = Command::new("qs").args(&base).stdout(Stdio::piped()).spawn()?;
    if let Some(mut out) = child.stdout.take() { let mut buf = String::new(); std::io::Read::read_to_string(&mut out, &mut buf)?; let filter = format!("Cannot open: file://{}/imagecache/", VPATHS.cache_dir.display()); for line in buf.lines() { if !line.contains(&filter) { println!("{}", line); } } }
    Ok(())
}

pub fn cmd_clipboard(args: ClipboardArgs) -> Result<()> {
    let out = Command::new("cliphist").arg("list").stdout(Stdio::piped()).output()?;
    let input = out.stdout;
    let mut fz = Command::new("fuzzel"); fz.arg("--dmenu"); if args.delete { fz.args(["--prompt=del > ", "--placeholder=Delete from clipboard"]); } else { fz.arg("--placeholder=Type to search clipboard"); }
    let mut child = fz.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?; child.stdin.as_mut().unwrap().write_all(&input)?; let picked = child.wait_with_output()?.stdout;
    if args.delete { let mut del = Command::new("cliphist").arg("delete").stdin(Stdio::piped()).spawn()?; del.stdin.as_mut().unwrap().write_all(&picked)?; del.wait()?; } else { let mut dec = Command::new("cliphist").arg("decode").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?; dec.stdin.as_mut().unwrap().write_all(&picked)?; let decoded = dec.wait_with_output()?.stdout; let mut wl = Command::new("wl-copy").stdin(Stdio::piped()).spawn()?; wl.stdin.as_mut().unwrap().write_all(&decoded)?; wl.wait()?; }
    Ok(())
}

pub fn cmd_screenshot(args: ScreenshotArgs) -> Result<()> {
    if let Some(region) = args.region.as_deref() {
        if region == "slurp" {
            let _ = Command::new("qs").args(["-c","vela","ipc","call", if args.freeze {"picker"} else {"picker"}, if args.freeze {"openFreeze"} else {"open"}]).status(); return Ok(());
        } else {
            let grim = Command::new("grim").args(["-l","0","-g",region.trim(),"-"]).stdout(Stdio::piped()).output()?; let mut sw = Command::new("swappy").args(["-f","-"]).stdin(Stdio::piped()).spawn()?; sw.stdin.as_mut().unwrap().write_all(&grim.stdout)?; return Ok(());
        }
    }
    let data = Command::new("grim").arg("-").stdout(Stdio::piped()).output()?.stdout; let mut wl = Command::new("wl-copy").stdin(Stdio::piped()).spawn()?; wl.stdin.as_mut().unwrap().write_all(&data)?;
    let ts = OffsetDateTime::now_utc().format(&time::format_description::parse("%Y%m%d%H%M%S").unwrap()).unwrap(); let dest = VPATHS.screenshots_cache_dir.join(&ts); ensure_parent(&dest)?; fs::write(&dest, &data)?;
    let act = notify(&["-i","image-x-generic-symbolic","-h", &format!("STRING:image-path:{}", dest.display()), "--action=open=Open","--action=save=Save", "Screenshot taken", &format!("Screenshot stored in {} and copied to clipboard", dest.display())])?;
    match act.as_str() { "open" => { let _ = Command::new("swappy").args(["-f"]).arg(dest).status(); }, "save" => { let new_dest = VPATHS.screenshots_dir.join(format!("{}.png", ts)); ensure_parent(&new_dest)?; fs::rename(&dest, &new_dest)?; let _ = notify(&["Screenshot saved", &format!("Saved to {}", new_dest.display())]); }, _ => {} }
    Ok(())
}

fn detect_recorder() -> String { let lspci = run_output("lspci", &[]).unwrap_or_default().to_lowercase(); if lspci.contains("nvidia") && which::which("wf-recorder").is_ok() { return "wf-recorder".into(); } if which::which("wl-screenrec").is_ok() { return "wl-screenrec".into(); } if which::which("wf-recorder").is_ok() { return "wf-recorder".into(); } "wl-screenrec".into() }

pub fn cmd_record(args: RecordArgs) -> Result<()> {
    let recorder = detect_recorder(); let running = Command::new("pidof").arg(&recorder).status()?.success(); if running { let _ = Command::new("pkill").arg(&recorder).status(); for _ in 0..100 { if !Command::new("pidof").arg(&recorder).status()?.success() { break; } std::thread::sleep(std::time::Duration::from_millis(100)); } let new_path = VPATHS.recordings_dir.join(format!("recording_{}.mp4", OffsetDateTime::now_utc().format(&time::format_description::parse("%Y%m%d_%H-%M-%S").unwrap()).unwrap())); ensure_parent(&new_path)?; if VPATHS.recording_path.exists() { fs::rename(&VPATHS.recording_path, &new_path)?; } let _ = notify(&["--action=watch=Watch","--action=open=Open","--action=delete=Delete","Recording stopped", &format!("Recording saved in {}", new_path.display())]); return Ok(()); }
    let mut rec_args: Vec<String> = vec![];
    if let Some(reg) = args.region.as_deref() { let r = if reg == "slurp" { run_output("slurp", &[])?.trim().to_string() } else { reg.to_string() }; rec_args.push("-g".into()); rec_args.push(r); } else { if let Ok(monitors_json) = run_output("hyprctl", &["monitors","-j"]) { if let Ok(val) = serde_json::from_str::<serde_json::Value>(&monitors_json) { if let Some(arr) = val.as_array() { if let Some(focused) = arr.iter().find(|m| m.get("focused").and_then(|b| b.as_bool()).unwrap_or(false)) { if let Some(name) = focused.get("name").and_then(|s| s.as_str()) { rec_args.push("-o".into()); rec_args.push(name.into()); } } } } } }
    if args.sound { let sources = run_output("pactl", &["list","short","sources"]).unwrap_or_default(); let mut audio_source: Option<String> = None; for line in sources.lines() { if line.contains("RUNNING") { audio_source = Some(line.split_whitespace().nth(1).unwrap_or("").to_string()); break; } } if audio_source.is_none() { for line in sources.lines() { if line.contains("IDLE") { audio_source = Some(line.split_whitespace().nth(1).unwrap_or("").to_string()); break; } } } let src = audio_source.ok_or_else(|| anyhow!("No audio source found"))?; if recorder == "wf-recorder" { rec_args.push(format!("--audio={}", src)); } else { rec_args.push("--audio".into()); rec_args.push("--audio-device".into()); rec_args.push(src); } }
    ensure_parent(&VPATHS.recording_path)?; let _ = Command::new(&recorder).args(&rec_args).arg("-f").arg(&VPATHS.recording_path).stderr(Stdio::piped()).spawn()?; let id = notify(&["-p","Recording started","Recording..."]) ?; ensure_parent(&VPATHS.recording_notif_path)?; fs::write(&VPATHS.recording_notif_path, id)?; Ok(())
}

pub fn cmd_emoji(args: EmojiArgs) -> Result<()> {
    if args.picker { let content = fs::read(&VPATHS.emojis_path).unwrap_or_default(); let mut fz = Command::new("fuzzel"); fz.arg("--dmenu").arg("--placeholder=Type to search emojis"); let mut child = fz.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?; child.stdin.as_mut().unwrap().write_all(&content)?; let out = child.wait_with_output()?.stdout; if let Some(first) = String::from_utf8_lossy(&out).split_whitespace().next() { let mut wl = Command::new("wl-copy").stdin(Stdio::piped()).spawn()?; wl.stdin.as_mut().unwrap().write_all(first.as_bytes())?; } return Ok(()); }
    if args.fetch { emoji_fetch()?; return Ok(()); }
    let content = fs::read_to_string(&VPATHS.emojis_path).unwrap_or_default(); print!("{}", content); Ok(())
}

fn emoji_fetch() -> Result<()> {
    let mut data: Vec<String> = vec!["¿? question upside down reversed spanish".into(), "← left arrow".into(), "↑ up arrow".into(), "→ right arrow".into(), "↓ down arrow".into(), "←↑→↓ all directions up down left right arrows".into(), "⇇ leftwards paired arrows".into(), "⇉ rightwards paired arrows".into(), "⇈ upwards paired arrows".into(), "⇊ downwards paired arrows".into(), "⬱ three leftwards arrows".into(), "⇶ three rightwards arrows".into(), "• dot circle separator".into(), "「」 japanese quote square bracket".into(), "¯\\_(ツ)_/¯ shrug idk i dont know".into(), "(ง🔥ﾛ🔥)ง person with fire eyes eyes on fire".into(), "↵ enter key return".into(), "° degrees".into(), "™ tm trademark".into(), "® registered trademark".into(), "© copyright".into(), "— em dash".into(), "󰖳 windows super key".into() ];
    let client = reqwest::blocking::Client::new(); let emojis_url = "https://raw.githubusercontent.com/milesj/emojibase/refs/heads/master/packages/data/en/compact.raw.json"; let emojis_val: serde_json::Value = client.get(emojis_url).send()?.error_for_status()?.json()?; if let Some(arr) = emojis_val.as_array() { for emoji in arr { let mut line: Vec<String> = vec![]; if let Some(u) = emoji.get("unicode").and_then(|v| v.as_str()) { line.push(u.into()); } if let Some(emoticon) = emoji.get("emoticon") { match emoticon { serde_json::Value::String(s) => line.push(s.clone()), serde_json::Value::Array(a) => { for v in a { if let Some(s) = v.as_str() { line.push(s.into()); } } }, _ => {} } } if let Some(label) = emoji.get("label").and_then(|v| v.as_str()) { line.push(label.into()); } if let Some(tags) = emoji.get("tags").and_then(|v| v.as_array()) { for t in tags { if let Some(s) = t.as_str() { line.push(s.into()); } } } if !line.is_empty() { data.push(line.join(" ")); } } }
    let glyphs_url = "https://raw.githubusercontent.com/ryanoasis/nerd-fonts/refs/heads/master/glyphnames.json"; let glyphs_val: serde_json::Value = client.get(glyphs_url).send()?.error_for_status()?.json()?; if let Some(obj) = glyphs_val.as_object() { let mut buckets: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new(); for (name, glyph) in obj { if name == "METADATA" { continue; } if let Some(char_val) = glyph.get("char").and_then(|v| v.as_str()) { buckets.entry(char_val.into()).or_default().push(format!("nf-{}", name)); } } for (ch, names) in buckets { data.push(format!("{}  {}", ch, names.join(" "))); } }
    ensure_parent(&VPATHS.emojis_path)?; fs::write(&VPATHS.emojis_path, data.join("\n"))?; Ok(())
}

pub fn cmd_wallpaper(args: WallpaperArgs) -> Result<()> {
    if let Some(p) = args.print { return wallpaper::cmd_wallpaper_print(Some(p)); }
    if let Some(dir) = args.random.or_else(|| Some(VPATHS.wallpapers_dir.clone())) { let mut list = wallpaper::wallpapers_in(&dir, args.no_filter, args.threshold)?; if list.is_empty() { return Err(anyhow!("No valid wallpapers found")); } if let Some(last) = wallpaper::wallpaper_get().map(PathBuf::from) { list.retain(|p| p != &last); if list.is_empty() { return Err(anyhow!("Only valid wallpaper is current")); } } if let Some(choice) = list.choose(&mut rand::thread_rng()) { wallpaper::wallpaper_set(choice, args.no_smart)?; } return Ok(()); }
    if let Some(file) = args.file { wallpaper::wallpaper_set(&file, args.no_smart)?; return Ok(()); }
    println!("{}", wallpaper::wallpaper_get().unwrap_or_else(|| "No wallpaper set".into())); Ok(())
}

pub fn cmd_toggle(workspace: String) -> Result<()> {
    // Minimal: toggle special workspace directly
    let _ = run("hyprctl", &["dispatch","togglespecialworkspace", &workspace]); Ok(())
}

pub fn cmd_resizer(_args: ResizerArgs) -> Result<()> { println!("Resizer daemon/tools not yet ported to Rust."); Ok(()) }

#[derive(clap::Args, Debug)]
pub struct EditorDetectArgs { #[arg(short, long)] pub path: Option<PathBuf> }

#[derive(clap::Args, Debug)]
pub struct EditorThemeArgs { #[arg(short, long)] pub path: Option<PathBuf> }

#[derive(clap::Subcommand, Debug)]
pub enum EditorCmd { Detect(EditorDetectArgs), Theme(EditorThemeArgs) }

#[derive(clap::Args, Debug)]
pub struct EditorArgs { #[command(subcommand)] pub cmd: EditorCmd }

pub fn cmd_editor(args: EditorArgs) -> Result<()> {
    match args.cmd {
        EditorCmd::Detect(a) => {
            let dir = a.path.unwrap_or_else(|| std::env::current_dir().unwrap());
            let counts = languages::detect_languages(&dir);
            let active = counts.iter().max_by_key(|(_, c)| **c).map(|(k, _)| k.clone()).unwrap_or_else(|| "unknown".into());
            languages::write_palette_override(&active)?;
            let out = serde_json::json!({"activeLanguage": active, "counts": counts});
            println!("{}", out.to_string());
            Ok(())
        }
        EditorCmd::Theme(_a) => {
            // Regenerate VSCode theme with current scheme colors
            let st = crate::scheme::scheme_state_load();
            crate::theme::apply_colours(&st.colors, &st.mode)?;
            println!("VSCodium theme written.");
            Ok(())
        }
    }
}
