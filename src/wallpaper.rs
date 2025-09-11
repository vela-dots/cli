use anyhow::{anyhow, Result};
use image::io::Reader as ImageReader;
use image::{imageops::FilterType, GenericImageView};
use serde_json::{json, Map, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

use crate::notify::notify;
use crate::paths::{VPATHS, ensure_parent};
use crate::scheme::{scheme_state_load, scheme_state_save, read_colors_from_file};
use crate::util::{hash_file, is_valid_image};

pub fn ensure_thumb(src: &Path, cache_dir: &Path) -> Result<PathBuf> {
    let thumb = cache_dir.join("thumbnail.jpg");
    if !thumb.exists() {
        ensure_parent(&thumb)?;
        let mut img = image::open(src)?.into_rgb8();
        let (mut w, mut h) = img.dimensions();
        while w > 128 || h > 128 { w = (w as f32 * 0.5) as u32; h = (h as f32 * 0.5) as u32; img = image::imageops::resize(&img, w.max(1), h.max(1), FilterType::Nearest); }
        img.save(&thumb)?;
    }
    Ok(thumb)
}

pub fn wallpaper_get() -> Option<String> { fs::read_to_string(&VPATHS.wallpaper_path_path).ok().map(|s| s.trim().to_string()) }

fn calc_colorfulness(img: &image::DynamicImage) -> f64 {
    let (w, h) = img.dimensions();
    let mut rg_diffs = Vec::with_capacity((w*h) as usize);
    let mut yb_diffs = Vec::with_capacity((w*h) as usize);
    for p in img.to_rgb8().pixels() {
        let r = p[0] as f64; let g = p[1] as f64; let b = p[2] as f64;
        let rg = (r - g).abs();
        let yb = ((r + g) * 0.5 - b).abs();
        rg_diffs.push(rg); yb_diffs.push(yb);
    }
    let mean = |v: &Vec<f64>| v.iter().copied().sum::<f64>() / (v.len() as f64);
    let stddev = |v: &Vec<f64>, m: f64| (v.iter().map(|x| (x - m)*(x - m)).sum::<f64>() / (v.len() as f64)).sqrt();
    let m_rg = mean(&rg_diffs); let m_yb = mean(&yb_diffs);
    let s_rg = stddev(&rg_diffs, m_rg); let s_yb = stddev(&yb_diffs, m_yb);
    (s_rg.powi(2) + s_yb.powi(2)).sqrt() + 0.3 * (m_rg.powi(2) + m_yb.powi(2)).sqrt()
}

fn detect_smart_opts(img: &image::DynamicImage) -> (String, String) {
    // Variant by colorfulness (match Python thresholds)
    let cf = calc_colorfulness(img);
    let variant = if cf < 10.0 { "neutral" } else if cf < 20.0 { "content" } else { "tonalspot" };
    // Mode by brightness of downscaled 1x1
    let mut i = img.clone();
    let i = image::imageops::resize(&i.to_rgb8(), 1, 1, FilterType::Lanczos3);
    let p = i.get_pixel(0, 0);
    let r = p[0] as f64; let g = p[1] as f64; let b = p[2] as f64;
    let y = 0.2126*r + 0.7152*g + 0.0722*b;
    let mode = if y > 155.0 { "light" } else { "dark" };
    (mode.into(), variant.into())
}

pub fn wallpaper_set(path: &Path, no_smart: bool) -> Result<()> {
    let path = path.canonicalize()?;
    if !is_valid_image(&path) { return Err(anyhow!(format!("\"{}\" is not a valid image", path.display()))); }
    ensure_parent(&VPATHS.wallpaper_path_path)?;
    fs::write(&VPATHS.wallpaper_path_path, path.display().to_string())?;
    ensure_parent(&VPATHS.wallpaper_link_path)?;
    let _ = fs::remove_file(&VPATHS.wallpaper_link_path);
    #[cfg(unix)]
    std::os::unix::fs::symlink(&path, &VPATHS.wallpaper_link_path)?;

    let cache = VPATHS.wallpapers_cache_dir.join(hash_file(&path)?);
    ensure_parent(&cache.join("dummy"))?;
    let thumb = ensure_thumb(&path, &cache)?;
    ensure_parent(&VPATHS.wallpaper_thumb_path)?;
    let _ = fs::remove_file(&VPATHS.wallpaper_thumb_path);
    #[cfg(unix)]
    std::os::unix::fs::symlink(&thumb, &VPATHS.wallpaper_thumb_path)?;

    let mut st = scheme_state_load();
    if st.name == "dynamic" && !no_smart {
        // compute smart options from thumbnail
        let img = ImageReader::open(&thumb)?.with_guessed_format()?.decode()?;
        let (mode, variant) = detect_smart_opts(&img);
        st.mode = mode; st.variant = variant;
    }
    // keep colors as-is for dynamic; static load from file
    if st.name != "dynamic" {
        let p = VPATHS.scheme_data_dir.join(&st.name).join(&st.flavor).join(format!("{}.txt", st.mode));
        st.colors = read_colors_from_file(&p)?;
    }
    scheme_state_save(&st)?;
    crate::theme::apply_colours(&st.colors, &st.mode)?;
    Ok(())
}

pub fn wallpapers_in(dir: &Path, no_filter: bool, _threshold: f32) -> Result<Vec<PathBuf>> {
    let mut out = vec![];
    if !dir.is_dir() { return Ok(out); }
    for e in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if e.file_type().is_file() {
            let p = e.path(); if is_valid_image(p) { out.push(p.to_path_buf()); }
        }
    }
    // TODO: implement monitor-based size filtering
    Ok(out)
}

pub fn cmd_wallpaper_print(path: Option<PathBuf>) -> Result<()> {
    let p = path.or_else(|| wallpaper_get().map(PathBuf::from));
    if let Some(p) = p { println!("{}", json!({"name":"dynamic","flavor":"default","mode":"dark","variant":"tonalspot","file":p}).to_string()); } else { println!("No wallpaper set"); }
    Ok(())
}

