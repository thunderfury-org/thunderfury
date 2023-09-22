use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;

use crate::common::error::Error;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub server: ServerConfig,

    #[serde(default)]
    pub library: LibraryConfig,

    #[serde(default)]
    pub provider: HashMap<String, ProviderConfig>,

    #[serde(default)]
    pub downloader: HashMap<String, DownloaderConfig>,

    #[serde(default)]
    pub message: MessageConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct ServerConfig {
    pub address: Option<String>,
    pub port: Option<u32>,
    pub db_url: Option<String>,
    pub disable_background_task: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
pub struct LibraryConfig {
    pub library_root: Option<String>,
    pub tmdb_api_key: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct MessageConfig {
    pub channel: HashMap<String, MessageChannelConfig>,
}

pub type ProviderConfig = HashMap<String, String>;
pub type DownloaderConfig = HashMap<String, String>;
pub type MessageChannelConfig = HashMap<String, String>;

#[derive(Clone)]
pub struct Manager {
    config_dir: Arc<String>,
    config: Arc<AppConfig>,
}

const DEFAULT_ADDRESS: &str = "0.0.0.0";
const DEFAULT_PORT: u32 = 8080;

pub const DEFAULT_LIBRARY_ROOT: &str = "/media/library";

impl Manager {
    pub fn get_config_dir(&self) -> &str {
        return self.config_dir.as_str();
    }
    pub fn get_server_config(&self) -> &ServerConfig {
        &self.config.server
    }

    pub fn get_server_address(&self) -> String {
        let c = self.get_server_config();
        format!(
            "{}:{}",
            c.address.as_deref().unwrap_or(DEFAULT_ADDRESS),
            c.port.unwrap_or(DEFAULT_PORT)
        )
    }

    pub fn get_library_config(&self) -> &LibraryConfig {
        &self.config.library
    }

    pub fn get_library_root(&self) -> &str {
        self.get_library_config()
            .library_root
            .as_deref()
            .unwrap_or(DEFAULT_LIBRARY_ROOT)
    }

    pub fn get_provider_config(&self, provider: &str) -> Option<&ProviderConfig> {
        self.config.provider.get(provider)
    }

    pub fn get_downloader_config(&self, downloader: &str) -> Option<&DownloaderConfig> {
        self.config.downloader.get(downloader)
    }

    pub fn get_message_channel_config(&self, channel: &str) -> Option<&MessageChannelConfig> {
        self.config.message.channel.get(channel)
    }
}

impl TryFrom<&str> for Manager {
    type Error = Error;

    fn try_from(config_dir: &str) -> Result<Self, Self::Error> {
        if config_dir.is_empty() {
            return Err(Error::Internal("config dir is empty".to_string()));
        }

        let config_file = format!("{config_dir}/config.yaml");

        crate::utils::fs::create_file_if_not_exists(config_file.as_str())?;

        match serde_yaml::from_str(std::fs::read_to_string(config_file.as_str())?.as_str()) {
            Ok(config) => Ok(Self {
                config_dir: Arc::new(config_dir.to_string()),
                config: Arc::new(config),
            }),
            Err(e) => Err(Error::Internal(format!("parse config file error, {}", e))),
        }
    }
}

impl From<AppConfig> for Manager {
    fn from(value: AppConfig) -> Self {
        Self {
            config_dir: Arc::new("".to_string()),
            config: Arc::new(value),
        }
    }
}
