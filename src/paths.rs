use once_cell::sync::Lazy;
use std::env;
use std::path::{PathBuf, Path};

#[derive(Clone, Debug)]
pub struct XdgPaths {
    pub config: PathBuf,
    pub data: PathBuf,
    pub state: PathBuf,
    pub cache: PathBuf,
    pub pictures: PathBuf,
    pub videos: PathBuf,
}

fn xdg_paths() -> XdgPaths {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    let config = env::var_os("XDG_CONFIG_HOME").map(PathBuf::from).unwrap_or(home.join(".config"));
    let data = env::var_os("XDG_DATA_HOME").map(PathBuf::from).unwrap_or(home.join(".local/share"));
    let state = env::var_os("XDG_STATE_HOME").map(PathBuf::from).unwrap_or(home.join(".local/state"));
    let cache = env::var_os("XDG_CACHE_HOME").map(PathBuf::from).unwrap_or(home.join(".cache"));
    let pictures = env::var_os("XDG_PICTURES_DIR").map(PathBuf::from).unwrap_or(home.join("Pictures"));
    let videos = env::var_os("XDG_VIDEOS_DIR").map(PathBuf::from).unwrap_or(home.join("Videos"));

    XdgPaths { config, data, state, cache, pictures, videos }
}

#[derive(Clone, Debug)]
pub struct VelaPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub state_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub user_config_path: PathBuf,
    pub templates_dir: PathBuf,
    pub user_templates_dir: PathBuf,
    pub theme_dir: PathBuf,
    pub scheme_path: PathBuf,
    pub scheme_data_dir: PathBuf,
    pub scheme_cache_dir: PathBuf,
    pub wallpapers_dir: PathBuf,
    pub wallpaper_path_path: PathBuf,
    pub wallpaper_link_path: PathBuf,
    pub wallpaper_thumb_path: PathBuf,
    pub wallpapers_cache_dir: PathBuf,
    pub screenshots_dir: PathBuf,
    pub screenshots_cache_dir: PathBuf,
    pub recordings_dir: PathBuf,
    pub recording_path: PathBuf,
    pub recording_notif_path: PathBuf,
    pub emojis_path: PathBuf,
}

fn workspace_root() -> Option<PathBuf> {
    let cwd = env::current_dir().ok()?;
    if cwd.join("cli/src/vela").exists() { return Some(cwd); }
    if cwd.join("src/vela").exists() { return Some(cwd); }
    if cwd.join("cli").exists() { return Some(cwd); }
    None
}

fn build_paths() -> VelaPaths {
    let xdg = xdg_paths();
    let v_config = xdg.config.join("vela");
    let v_data = xdg.data.join("vela");
    let v_state = xdg.state.join("vela");
    let v_cache = xdg.cache.join("vela");

    let (templates_dir, scheme_data_dir, emojis_path) = if let Some(root) = workspace_root() {
        let cli_data = root.join("cli/src/vela/data");
        (
            cli_data.join("templates"),
            cli_data.join("schemes"),
            cli_data.join("emojis.txt"),
        )
    } else {
        (
            v_config.join("templates"),
            v_data.join("schemes"),
            v_data.join("emojis.txt"),
        )
    };

    let wallpapers_dir = env::var_os("VELA_WALLPAPERS_DIR").map(PathBuf::from).unwrap_or(xdg.pictures.join("Wallpapers"));
    let screenshots_dir = env::var_os("VELA_SCREENSHOTS_DIR").map(PathBuf::from).unwrap_or(xdg.pictures.join("Screenshots"));
    let recordings_dir = env::var_os("VELA_RECORDINGS_DIR").map(PathBuf::from).unwrap_or(xdg.videos.join("Recordings"));

    VelaPaths {
        config_dir: v_config.clone(),
        data_dir: v_data.clone(),
        state_dir: v_state.clone(),
        cache_dir: v_cache.clone(),
        user_config_path: v_config.join("cli.json"),
        templates_dir: templates_dir.clone(),
        user_templates_dir: v_config.join("templates"),
        theme_dir: v_state.join("theme"),
        scheme_path: v_state.join("scheme.json"),
        scheme_data_dir: scheme_data_dir.clone(),
        scheme_cache_dir: v_cache.join("schemes"),
        wallpapers_dir,
        wallpaper_path_path: v_state.join("wallpaper/path.txt"),
        wallpaper_link_path: v_state.join("wallpaper/current"),
        wallpaper_thumb_path: v_state.join("wallpaper/thumbnail.jpg"),
        wallpapers_cache_dir: v_cache.join("wallpapers"),
        screenshots_dir: screenshots_dir.clone(),
        screenshots_cache_dir: v_cache.join("screenshots"),
        recordings_dir: recordings_dir.clone(),
        recording_path: v_state.join("record/recording.mp4"),
        recording_notif_path: v_state.join("record/notifid.txt"),
        emojis_path,
    }
}

pub static VPATHS: Lazy<VelaPaths> = Lazy::new(build_paths);

pub fn ensure_parent(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}
