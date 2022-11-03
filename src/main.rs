use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self},
    path::{Path, PathBuf},
    time::SystemTime,
};

const CONFIG_PATH: &str = "/etc/hubakc/config.toml";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required(true))]
    user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ttl: u64,
    cache_folder: String,
    user_map: HashMap<String, String>, // user -> gh user
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache_folder: "/tmp/hubakc".to_string(),
            user_map: Default::default(),
            ttl: 600,
        }
    }
}

impl Config {
    fn from_path(p: &str) -> Self {
        toml::de::from_str(fs::read_to_string(p).unwrap().as_str()).unwrap()
    }
}

fn get_pubkey_from_gh(gh_user_name: &str) -> String {
    ureq::get(&format!("https://github.com/{gh_user_name}.keys"))
        .call()
        .unwrap()
        .into_string()
        .unwrap()
}

fn main() {
    let Args { user_name } = Args::parse();
    let config = Config::from_path(CONFIG_PATH);
    let cache_dir_path = Path::new(&config.cache_folder);
    if !cache_dir_path.exists() {
        fs::create_dir_all(cache_dir_path).unwrap();
    }
    if let Some(gh_user) = config.user_map.get(&user_name) {
        let cache_path = PathBuf::from(format!("{}/{}", config.cache_folder, gh_user));
        if cache_path.exists() {
            let mod_time = cache_path.metadata().unwrap().modified().unwrap();
            if SystemTime::now()
                .duration_since(mod_time)
                .unwrap()
                .as_secs()
                <= config.ttl
            {
                print!("{}", fs::read_to_string(cache_path).unwrap());
                return;
            }
        }
        let keys = get_pubkey_from_gh(&gh_user);
        print!("{}", keys);
        fs::write(cache_path, keys).unwrap();
    }
}
