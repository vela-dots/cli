use anyhow::Result;
use serde_json::Map;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::Write;

use crate::paths::{VPATHS, ensure_parent};

fn c2s(c: &str, idx: &[u8]) -> String {
    let rgb = format!("{}/{}/{}", &c[0..2], &c[2..4], &c[4..6]);
    let mut s = String::new();
    if idx.len() == 1 { s.push_str(&format!("\x1b]{};rgb:{}\x1b\\", idx[0], rgb)); }
    else { s.push_str(&format!("\x1b]{};{};rgb:{}\x1b\\", idx[0], idx[1], rgb)); }
    s
}

fn gen_sequences(colors: &Map<String, serde_json::Value>) -> String {
    // minimal set used by Python
    let g = |k: &str| colors.get(k).and_then(|v| v.as_str()).unwrap_or("000000");
    [
        c2s(g("onSurface"), &[10]),
        c2s(g("surface"), &[11]),
        c2s(g("secondary"), &[12]),
        c2s(g("secondary"), &[17]),
        c2s(g("term0"), &[4,0]), c2s(g("term1"), &[4,1]), c2s(g("term2"), &[4,2]), c2s(g("term3"), &[4,3]),
        c2s(g("term4"), &[4,4]), c2s(g("term5"), &[4,5]), c2s(g("term6"), &[4,6]), c2s(g("term7"), &[4,7]),
        c2s(g("term8"), &[4,8]), c2s(g("term9"), &[4,9]), c2s(g("term10"), &[4,10]), c2s(g("term11"), &[4,11]),
        c2s(g("term12"), &[4,12]), c2s(g("term13"), &[4,13]), c2s(g("term14"), &[4,14]), c2s(g("term15"), &[4,15]),
        c2s(g("primary"), &[4,16]), c2s(g("secondary"), &[4,17]), c2s(g("tertiary"), &[4,18]),
    ].join("")
}

fn write_file(path: &Path, content: &str) -> Result<()> { ensure_parent(path)?; fs::write(path, content)?; Ok(()) }

pub fn apply_terms(sequences: &str) -> Result<()> {
    let state = VPATHS.state_dir.join("sequences.txt");
    write_file(&state, sequences)?;
    // broadcast to /dev/pts/*
    if let Ok(dir) = fs::read_dir("/dev/pts") {
        for e in dir.flatten() {
            let p = e.path(); if p.file_name().and_then(|s| s.to_str()).map(|s| s.chars().all(|c| c.is_ascii_digit())).unwrap_or(false) {
                if let Ok(mut f) = fs::OpenOptions::new().append(true).open(&p) { let _ = f.write_all(sequences.as_bytes()); }
            }
        }
    }
    Ok(())
}

pub fn apply_hypr(conf: &str) -> Result<()> { write_file(&VPATHS.config_dir.join("hypr/scheme/current.conf"), conf) }

pub fn gen_conf(colors: &Map<String, serde_json::Value>) -> String {
    let mut s = String::new();
    for (k, v) in colors { if let Some(c) = v.as_str() { s.push_str(&format!("${} = {}\n", k, c)); } }
    s
}

pub fn gen_replace(template: &Path, colors: &Map<String, serde_json::Value>, hash: bool) -> Result<String> {
    let mut t = fs::read_to_string(template)?;
    for (k, v) in colors { if let Some(c) = v.as_str() { let repl = if hash { format!("#{c}") } else { c.to_string() }; t = t.replace(&format!("{{{{ ${} }}}}", k), &repl); } }
    Ok(t)
}

pub fn apply_colours(colors: &Map<String, serde_json::Value>, mode: &str) -> Result<()> {
    // Read ~/.config/vela/cli.json theme config
    let cfg = std::fs::read_to_string(&VPATHS.user_config_path).ok().and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok()).and_then(|v| v.get("theme").cloned()).unwrap_or(serde_json::json!({}));
    let check = |k: &str| cfg.get(k).and_then(|v| v.as_bool()).unwrap_or(true);

    if check("enableTerm") { apply_terms(&gen_sequences(colors))?; }
    if check("enableHypr") { apply_hypr(&gen_conf(colors))?; }
    if check("enableFuzzel") { let s = gen_replace(&VPATHS.templates_dir.join("fuzzel.ini"), colors, true)?; write_file(&VPATHS.config_dir.join("fuzzel/fuzzel.ini"), &s)?; }
    if check("enableBtop") { let s = gen_replace(&VPATHS.templates_dir.join("btop.theme"), colors, true)?; write_file(&VPATHS.config_dir.join("btop/themes/vela.theme"), &s)?; let _ = Command::new("killall").args(["-USR2","btop"]).stderr(Stdio::null()).status(); }
    if check("enableNvtop") { let s = gen_replace(&VPATHS.templates_dir.join("nvtop.colors"), colors, true)?; write_file(&VPATHS.config_dir.join("nvtop/nvtop.colors"), &s)?; }
    if check("enableHtop") { let s = gen_replace(&VPATHS.templates_dir.join("htop.theme"), colors, true)?; write_file(&VPATHS.config_dir.join("htop/htoprc"), &s)?; let _ = Command::new("killall").args(["-USR2","htop"]).stderr(Stdio::null()).status(); }
    if check("enableCava") { let s = gen_replace(&VPATHS.templates_dir.join("cava.conf"), colors, true)?; write_file(&VPATHS.config_dir.join("cava/config"), &s)?; let _ = Command::new("killall").args(["-USR2","cava"]).stderr(Stdio::null()).status(); }
    // VSCode/VSCodium theme
    if check("enableVscode") {
        let mut s = gen_replace(&VPATHS.templates_dir.join("vscode-theme.json"), colors, true)?;
        s = s.replace("${mode}", mode);
        let target = dirs::config_dir().unwrap_or(VPATHS.config_dir.clone()).join("VSCodium/User/themes/vela-color-theme.json");
        write_file(&target, &s)?;
    }
    // Helix theme
    if check("enableHelix") {
        let s = gen_replace(&VPATHS.templates_dir.join("helix.toml"), colors, true)?;
        let target = VPATHS.config_dir.join("helix/themes/vela.toml");
        write_file(&target, &s)?;
    }
    // VSCode/VSCodium theme
    if check("enableVscode") {
        let mut s = gen_replace(&VPATHS.templates_dir.join("vscode-theme.json"), colors, true)?;
        s = s.replace("${mode}", mode);
        let target = dirs::config_dir().unwrap_or(VPATHS.config_dir.clone()).join("VSCodium/User/themes/vela-color-theme.json");
        write_file(&target, &s)?;
    }
    if check("enableGtk") { let s = gen_replace(&VPATHS.templates_dir.join("gtk.css"), colors, true)?; write_file(&VPATHS.config_dir.join("gtk-3.0/gtk.css"), &s)?; write_file(&VPATHS.config_dir.join("gtk-4.0/gtk.css"), &s)?; let _ = Command::new("dconf").args(["write","/org/gnome/desktop/interface/color-scheme", &format!("'prefer-{}'", mode)]).status(); }
    if check("enableQt") { let s = gen_replace(&VPATHS.templates_dir.join(format!("qt{}.colors", mode)), colors, true)?; write_file(&VPATHS.config_dir.join("qt5ct/colors/vela.colors"), &s)?; write_file(&VPATHS.config_dir.join("qt6ct/colors/vela.colors"), &s)?; }
    if check("enableWarp") { let mut s = gen_replace(&VPATHS.templates_dir.join("warp.yaml"), colors, true)?; let warp_mode = if mode=="dark" { "darker" } else { "lighter" }; s = s.replace("{{ $warp_mode }}", warp_mode); write_file(&VPATHS.data_dir.join("warp-terminal/themes/vela.yaml"), &s)?; }
    // Discord (sass) and user_templates omitted or TODO
    Ok(())
}
