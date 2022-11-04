use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};
const CONFIG_PATH: &str = "/etc/hubakc/config.toml";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ttl: u64,
    timeout: u64,
    cache_folder: String,
    user_map: HashMap<String, String>, // user -> gh user
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache_folder: "/tmp/hubakc".to_string(),
            user_map: Default::default(),
            ttl: 600,
            timeout: 15,
        }
    }
}

impl Config {
    fn from_path(p: &str) -> Result<Self> {
        let res: Self = toml::from_str(fs::read_to_string(p)?.as_str())?;
        Ok(res)
    }
}

fn get_pubkey_from_gh(gh_user_name: &str, timeout: u64) -> Result<String> {
    let res = minreq::get(&format!("https://github.com/{gh_user_name}.keys"))
        .with_timeout(timeout)
        .send()?
        .as_str()?
        .to_string();
    Ok(res)
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print!("A simple tool to grab ssh public key from GitHub\n\nUsage: hubakc <USERNAME>\n");
        return Ok(());
    }
    ensure!(args.len() == 2, "expect exactly 1 args");
    let user_name: &str = &args[1];
    let config = Config::from_path(CONFIG_PATH)
        .with_context(|| format!("failed to opening config file: {CONFIG_PATH}"))?;
    let cache_dir_path = Path::new(&config.cache_folder);
    if !cache_dir_path.exists() {
        fs::create_dir_all(cache_dir_path)
            .with_context(|| format!("failed to create cache dir: {}", cache_dir_path.display()))?;
    }
    if let Some(gh_user) = config.user_map.get(user_name) {
        let cache_path = PathBuf::from(format!("{}/{}", config.cache_folder, gh_user));
        if cache_path.exists() {
            let mod_time = cache_path
                .metadata()
                .with_context(|| format!("failed to read metadata: {}", cache_path.display()))?
                .modified()?;
            if SystemTime::now().duration_since(mod_time)?.as_secs() <= config.ttl
                || get_pubkey_from_gh(gh_user, config.timeout).is_err()
            {
                print!("{}", fs::read_to_string(cache_path)?);
                return Ok(());
            }
        }
        let keys = get_pubkey_from_gh(gh_user, config.timeout)
            .with_context(|| format!("failed to get ssh key for {gh_user}"))?;
        print!("{}", keys);
        fs::write(&cache_path, keys).with_context(|| {
            format!(
                "failed to write keys to cache file: {}",
                cache_path.display()
            )
        })?;
    }
    Ok(())
}
