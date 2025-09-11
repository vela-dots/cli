use anyhow::Result;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use serde_json::json;

use crate::paths::{VPATHS, ensure_parent};

fn ext_to_lang(ext: &str) -> Option<&'static str> {
    match ext.to_lowercase().as_str() {
        "rs" => Some("rust"),
        "svelte" => Some("svelte"),
        "qml" => Some("qml"),
        "js" => Some("javascript"),
        "cjs"|"mjs" => Some("javascript"),
        "ts" => Some("typescript"),
        "tsx" => Some("typescript"),
        "astro" => Some("astro"),
        "py" => Some("python"),
        "lua" => Some("lua"),
        "html"|"htm" => Some("html"),
        "scss"|"sass" => Some("scss"),
        "css" => Some("css"),
        "json" => Some("json"),
        _ => None
    }
}

pub fn detect_languages(path: &Path) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
                if let Some(lang) = ext_to_lang(ext) {
                    *counts.entry(lang.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    counts
}

fn accent_for_language(lang: &str) -> Option<&'static str> {
    match lang {
        "svelte" => Some("ff3e00"),
        "rust" => Some("d28445"),
        "qml" => Some("41cd52"),
        "typescript" => Some("3178c6"),
        "javascript" => Some("f7df1e"),
        "python" => Some("3776ab"),
        "lua" => Some("2c2d72"),
        "astro" => Some("ff5d01"),
        "html" => Some("e34f26"),
        "scss" => Some("c6538c"),
        _ => None,
    }
}

pub fn write_palette_override(active_lang: &str) -> Result<()> {
    if let Some(primary) = accent_for_language(active_lang) {
        let data = json!({
            "activeLanguage": active_lang,
            "colors": {
                "primary": primary,
                "surfaceTint": primary
            }
        });
        let path = VPATHS.state_dir.join("palette_override.json");
        ensure_parent(&path)?;
        fs::write(&path, serde_json::to_string_pretty(&data)?)?;
    }
    Ok(())
}

