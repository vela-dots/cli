use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::paths::{VPATHS, ensure_parent};
use crate::util::run;
use crate::theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemeState {
    pub name: String,
    pub flavor: String,
    pub mode: String,
    pub variant: String,
    pub colors: Map<String, Value>,
}

impl Default for SchemeState {
    fn default() -> Self {
        Self { name: "svelte".into(), flavor: "default".into(), mode: "dark".into(), variant: "tonalspot".into(), colors: Map::new() }
    }
}

pub fn read_colors_from_file(path: &Path) -> Result<Map<String, Value>> {
    let mut map = Map::new();
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        if line.trim().is_empty() { continue; }
        let mut it = line.split_whitespace();
        if let (Some(k), Some(v)) = (it.next(), it.next()) { map.insert(k.to_string(), Value::String(v.to_string())); }
    }
    Ok(map)
}

pub fn scheme_state_load() -> SchemeState {
    if let Ok(s) = fs::read_to_string(&VPATHS.scheme_path) { if let Ok(val) = serde_json::from_str(&s) { return val; } }
    let def = SchemeState::default();
    let _ = ensure_parent(&VPATHS.scheme_path);
    let _ = fs::write(&VPATHS.scheme_path, serde_json::to_string_pretty(&def).unwrap());
    def
}

pub fn scheme_state_save(s: &SchemeState) -> Result<()> { ensure_parent(&VPATHS.scheme_path)?; fs::write(&VPATHS.scheme_path, serde_json::to_string_pretty(s)?)?; Ok(()) }

pub fn get_scheme_names() -> Vec<String> {
    let mut v = vec![];
    if VPATHS.scheme_data_dir.exists() {
        if let Ok(rd) = fs::read_dir(&VPATHS.scheme_data_dir) {
            for e in rd.flatten() { if e.file_type().map(|t| t.is_dir()).unwrap_or(false) { v.push(e.file_name().to_string_lossy().to_string()); } }
        }
    }
    v.push("dynamic".into());
    v
}

pub fn get_scheme_flavors(name: Option<&str>) -> Vec<String> {
    let name: String = name.map(|s| s.to_string()).unwrap_or_else(|| scheme_state_load().name);
    if name == "dynamic" { return vec!["default".into()]; }
    let mut v = vec![];
    let d = VPATHS.scheme_data_dir.join(name);
    if let Ok(rd) = fs::read_dir(d) {
        for e in rd.flatten() { if e.file_type().map(|t| t.is_dir()).unwrap_or(false) { v.push(e.file_name().to_string_lossy().to_string()); } }
    }
    v
}

pub fn get_scheme_modes(name: Option<&str>, flavor: Option<&str>) -> Vec<String> {
    let (name, flavor) = if name.is_none() || flavor.is_none() {
        let st = scheme_state_load(); (st.name, st.flavor)
    } else { (name.unwrap().to_string(), flavor.unwrap().to_string()) };
    if name == "dynamic" { return vec!["light".into(), "dark".into()]; }
    let mut v = vec![];
    let d = VPATHS.scheme_data_dir.join(name).join(flavor);
    if let Ok(rd) = fs::read_dir(d) { for e in rd.flatten() { if e.file_type().map(|t| t.is_file()).unwrap_or(false) { if let Some(stem) = e.path().file_stem().and_then(|s| s.to_str()) { v.push(stem.to_string()); } } } }
    v
}

pub fn scheme_list(names: bool, flavors: bool, modes: bool, variants: bool) -> Result<()> {
    let names_list = get_scheme_names();
    let variants_list = vec!["tonalspot","vibrant","expressive","fidelity","fruitsalad","monochrome","neutral","rainbow","content"];
    let mut printed = false;
    if names { println!("{}", names_list.join("\n")); printed = true; }
    if flavors {
        let mut flv: Vec<String> = vec![];
        for n in &names_list { if n == "dynamic" { continue; } flv.extend(get_scheme_flavors(Some(n))); }
        flv.sort(); flv.dedup(); println!("{}", flv.join("\n")); printed = true;
    }
    if modes { println!("dark\nlight"); printed = true; }
    if variants { println!("{}", variants_list.join("\n")); printed = true; }
    if !printed {
        // Print compact map scheme->flavor->colors for current mode/variant
        let st = scheme_state_load();
        let mut out: BTreeMap<String, BTreeMap<String, Map<String, Value>>> = BTreeMap::new();
        for name in names_list {
            if name == "dynamic" { continue; }
            let mut inner = BTreeMap::new();
            for fl in get_scheme_flavors(Some(&name)) {
                let mut s = SchemeState { name: name.clone(), flavor: fl.clone(), mode: st.mode.clone(), variant: st.variant.clone(), colors: st.colors.clone() };
                let modes = get_scheme_modes(Some(&s.name), Some(&s.flavor));
                if !modes.contains(&s.mode) { if let Some(first) = modes.first() { s.mode = first.clone(); } }
                if let Ok(cols) = read_colors_from_file(&VPATHS.scheme_data_dir.join(&s.name).join(&s.flavor).join(format!("{}.txt", s.mode))) { inner.insert(fl.clone(), cols); }
            }
            out.insert(name, inner);
        }
        println!("{}", serde_json::to_string(&out)?);
    }
    Ok(())
}

pub fn scheme_get(name: bool, flavor: bool, mode: bool, variant: bool) -> Result<()> {
    let st = scheme_state_load();
    if name | flavor | mode | variant {
        if name { println!("{}", st.name); }
        if flavor { println!("{}", st.flavor); }
        if mode { println!("{}", st.mode); }
        if variant { println!("{}", st.variant); }
    } else {
        println!("Current scheme:\n    Name: {}\n    Flavor: {}\n    Mode: {}\n    Variant: {}", st.name, st.flavor, st.mode, st.variant);
    }
    Ok(())
}

#[derive(clap::Args, Debug)]
pub struct SchemeSetArgs {
    #[arg(long)] pub notify: bool,
    #[arg(short, long)] pub random: bool,
    #[arg(short, long)] pub name: Option<String>,
    #[arg(short = 'f', long = "flavor")] pub flavor: Option<String>,
    #[arg(short, long, value_parser = ["dark", "light"]) ] pub mode: Option<String>,
    #[arg(short, long)] pub variant: Option<String>,
}

pub fn scheme_set(args: SchemeSetArgs) -> Result<()> {
    let mut st = scheme_state_load();
    if args.random {
        let mut names = get_scheme_names(); names.retain(|n| n != "dynamic");
        if let Some(choice) = names.choose(&mut rand::thread_rng()) { st.name = choice.clone(); }
        let fl = get_scheme_flavors(Some(&st.name)); if let Some(c) = fl.choose(&mut rand::thread_rng()) { st.flavor = c.clone(); }
        let md = get_scheme_modes(Some(&st.name), Some(&st.flavor)); if let Some(c) = md.choose(&mut rand::thread_rng()) { st.mode = c.clone(); }
    }
    if let Some(n) = args.name { st.name = n; }
    if let Some(f) = args.flavor { st.flavor = f; }
    if let Some(m) = args.mode { st.mode = m; }
    if let Some(v) = args.variant { st.variant = v; }

    // Load colors from file unless dynamic
    if st.name != "dynamic" {
        let path = VPATHS.scheme_data_dir.join(&st.name).join(&st.flavor).join(format!("{}.txt", &st.mode));
        st.colors = read_colors_from_file(&path)?;
    }
    scheme_state_save(&st)?;
    // Apply theme (partial parity)
    theme::apply_colours(&st.colors, &st.mode)?;
    Ok(())
}
