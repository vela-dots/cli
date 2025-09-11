use anyhow::Result;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use serde_json::json;

use crate::paths::{VPATHS, ensure_parent};

fn ext_to_lang(ext: &str) -> Option<&'static str> {
    match ext.to_lowercase().as_str() {
        // Web/JS ecosystem
        "js" | "cjs" | "mjs" => Some("javascript"),
        "ts" | "tsx" => Some("typescript"),
        "jsx" => Some("javascript"),
        "vue" => Some("vue"),
        "svelte" => Some("svelte"),
        "astro" => Some("astro"),
        "scss" | "sass" => Some("scss"),
        "css" => Some("css"),
        "html" | "htm" => Some("html"),

        // Systems
        "rs" => Some("rust"),
        "c" => Some("c"),
        "h" => Some("c"),
        "cc" | "cpp" | "cxx" | "hpp" => Some("cpp"),
        "zig" => Some("zig"),
        "go" => Some("go"),
        "nim" => Some("nim"),
        "nix" => Some("nix"),

        // Mobile/backend
        "java" => Some("java"),
        "kt" | "kts" => Some("kotlin"),
        "swift" => Some("swift"),
        "dart" => Some("dart"),

        // Scripting/data
        "py" => Some("python"),
        "lua" => Some("lua"),
        "rb" => Some("ruby"),
        "php" => Some("php"),
        "sh" | "bash" | "zsh" | "fish" => Some("shell"),
        "yml" | "yaml" => Some("yaml"),
        "toml" => Some("toml"),
        "json" | "jsonc" => Some("json"),
        "xml" => Some("xml"),
        "md" | "markdown" | "mdx" => Some("markdown"),
        "sql" => Some("sql"),

        // FP/BEAM
        "ex" | "exs" => Some("elixir"),
        "erl" => Some("erlang"),
        "hs" => Some("haskell"),
        "ml" | "mli" => Some("ocaml"),

        // Others
        "qml" => Some("qml"),
        _ => None,
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

fn accent_for_language(lang: &str) -> Option<(&'static str, &'static str, &'static str)> {
    match lang {
        // primary, secondary, tertiary accents
        "svelte" => Some(("ff3e00", "312e81", "1f2937")),
        "rust" => Some(("d28445", "b294bb", "81a2be")),
        "qml" => Some(("41cd52", "3ea8ff", "ff7a93")),
        "typescript" => Some(("3178c6", "42a5f5", "90caf9")),
        "javascript" => Some(("f7df1e", "f59e0b", "22c55e")),
        "python" => Some(("3776ab", "ffd43b", "646464")),
        "lua" => Some(("2c2d72", "51a0d5", "ebb55b")),
        "astro" => Some(("ff5d01", "00b0ff", "7c4dff")),
        "html" => Some(("e34f26", "264de4", "f16529")),
        "scss" => Some(("c6538c", "563d7c", "ff79c6")),
        "css" => Some(("2965f1", "563d7c", "7cb342")),
        "vue" => Some(("41b883", "35495e", "00b3ff")),
        "go" => Some(("00add8", "fdb827", "375eab")),
        "c" => Some(("00599c", "a8b9cc", "f34b7d")),
        "cpp" => Some(("00599c", "f34b7d", "a8b9cc")),
        "zig" => Some(("f7a41d", "b8bb26", "83a598")),
        "java" => Some(("5382a1", "f8981d", "007396")),
        "kotlin" => Some(("7f52ff", "fdb827", "01d1ff")),
        "swift" => Some(("f05138", "ff9f0a", "34c759")),
        "dart" => Some(("0175c2", "13b9fd", "0ea5e9")),
        "shell" => Some(("3c873a", "ffd43b", "6aa84f")),
        "yaml" => Some(("cbcb41", "00acc1", "8bc34a")),
        "toml" => Some(("9c27b0", "ff7043", "26a69a")),
        "markdown" => Some(("4caf50", "ffb300", "42a5f5")),
        "json" => Some(("fbc02d", "03a9f4", "8bc34a")),
        "sql" => Some(("e67e22", "2980b9", "27ae60")),
        "elixir" => Some(("4b275f", "7e57c2", "ce93d8")),
        "erlang" => Some(("a90533", "ef5350", "ab47bc")),
        "haskell" => Some(("5e5086", "8e44ad", "3498db")),
        "ocaml" => Some(("ef7a08", "3775a9", "a7a8ad")),
        _ => None,
    }
}

pub fn write_palette_override(active_lang: &str) -> Result<()> {
    if let Some((primary, secondary, tertiary)) = accent_for_language(active_lang) {
        let data = json!({
            "activeLanguage": active_lang,
            "colors": {
                "primary": primary,
                "secondary": secondary,
                "tertiary": tertiary,
                "surfaceTint": primary
            }
        });
        let path = VPATHS.state_dir.join("palette_override.json");
        ensure_parent(&path)?;
        fs::write(&path, serde_json::to_string_pretty(&data)?)?;
    }
    Ok(())
}
