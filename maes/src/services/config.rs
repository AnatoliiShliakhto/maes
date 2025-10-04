use crate::prelude::*;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, RwLock},
};

static CONFIG_STATE: LazyLock<Arc<RwLock<ConfigState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(ConfigService::init_state())));

struct ConfigState {
    config: Config,
    path: Option<PathBuf>,
}

#[derive(Copy, Clone)]
pub struct ConfigService;

impl ConfigService {
    fn init_state() -> ConfigState {
        let path = dirs::data_dir().map(|p| p.join(env!("APP_NAME")).join("config.json"));
        let config = match &path {
            Some(p) => Self::load_file(p).unwrap_or_else(|e| {
                error!("{e}");
                Self::default_config()
            }),
            None => Self::default_config(),
        };
        ConfigState { config, path }
    }

    pub fn read() -> Config {
        if let Ok(guard) = CONFIG_STATE.read() {
            guard.config.clone()
        } else {
            Self::default_config()
        }
    }

    pub fn with_mut<F, R>(f: F) -> Result<R>
    where
        F: FnOnce(&mut Config) -> R,
    {
        let arc = &*CONFIG_STATE;
        let mut state = arc
            .write()
            .map_err(|e| format!("{e}"))?;
        let result = f(&mut state.config);
        if let Some(path) = &state.path {
            Self::save_file_atomic(&state.config, path)?;
        }
        Ok(result)
    }
    
    fn default_config() -> Config {
        Config {
            server: ServerConfig {
                ident: safe_nanoid!(10),
                host: "http://192.168.137.1:4583".to_string(),
                remote: false,
            },
            wifi: WiFiConfig {
                ssid: format!("maes-{}", safe_nanoid!(4)),
                password: "12345678".to_string(),
                start: false,
            },
            recent: RecentConfig {
                workspace: "".to_string(),
                login: "".to_string(),
                export: dirs::desktop_dir().unwrap_or_default(),
                import: dirs::desktop_dir().unwrap_or_default(),
            },
            main_window: Default::default(),
            child_window: Default::default(),
        }
    }

    fn load_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let path = path.as_ref();
        if !path.exists() {
            return Err("Config path not exists".into());
        }
        let content = fs::read_to_string(path).map_err(|e| format!("{e}"))?;
        let config = serde_json::from_str(&content).map_err(|e| format!("{e}"))?;
        Ok(config)
    }

    fn save_file_atomic<P: AsRef<Path>>(config: &Config, path: P) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("{e}"))?;
        }

        let tmp_path = path.with_extension("json.tmp");
        let json = serde_json::to_string_pretty(config).map_err(|e| format!("{e}"))?;

        {
            let mut f = fs::File::create(&tmp_path).map_err(|e| format!("{e}"))?;
            f.write_all(json.as_bytes()).map_err(|e| format!("{e}"))?;
            f.sync_all().ok();
        }

        fs::rename(&tmp_path, path).map_err(|e| {
            let _ = fs::remove_file(&tmp_path);
            format!("{e}")
        })?;

        Ok(())
    }
}