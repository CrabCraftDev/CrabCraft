use crate::config::configs::main::MainConfig;
use crate::config::migrations;
use crate::utils::errors::Error;
use crate::utils::files::FILES;
use crab_config::SemverVersion;
use crab_config::configurable::Configurable;
use crab_config::error::ConfigError;
use crab_config::handlers::format_handler::ConfigFormatHandler;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::types::config::Config;
use crossbeam::sync::{ShardedLock, ShardedLockReadGuard, ShardedLockWriteGuard};
use std::path::PathBuf;
use std::sync::OnceLock;
use tracing::error;

static CONFIG_MANAGER: OnceLock<ShardedLock<ConfigManager>> = OnceLock::new();

#[derive(Debug)]
pub struct ConfigManager {
    #[allow(unused)]
    version: String,
    pub main: MainConfig,
}

impl ConfigManager {
    pub fn init() -> Result<(), Error> {
        let config_path = FILES.config.join("main.yml");
        let yaml_handler: YamlHandler = YamlHandler {};

        let configurable = Configurable::<MainConfig, SemverVersion>::new(
            config_path.to_str().unwrap().to_string(),
            Box::new(yaml_handler),
        );

        let config = match configurable.load_or_create_default_config(migrations::list()) {
            Ok(config) => config,
            Err(error) => {
                error!("Error while loading config files");

                if let ConfigError::InvalidConfig { errors } = &error {
                    for err in errors {
                        error!("- {}", err.to_string());
                    }
                } else {
                    error!("{:?}", error);
                }
                error!("");
                return Err(Error::Config(error));
            }
        };

        CONFIG_MANAGER.get_or_init(|| {
            ShardedLock::new(Self {
                version: config._version.clone(),
                main: config,
            })
        });

        Ok(())
    }

    pub fn get() -> ShardedLockReadGuard<'static, ConfigManager> {
        CONFIG_MANAGER.get().unwrap().read().unwrap()
    }

    pub fn get_mut() -> ShardedLockWriteGuard<'static, ConfigManager> {
        CONFIG_MANAGER.get().unwrap().write().unwrap()
    }

    pub fn save(&self) {
        self.save_single(Self::get_config_path("main.yml"), &self.main);
    }

    fn save_single(&self, path: PathBuf, config: &impl Config) {
        YamlHandler {}
            .write(path.to_str().unwrap().to_string(), config.as_configuration_node())
            .unwrap()
    }

    fn get_config_path(name: &str) -> PathBuf {
        FILES.config.join(name)
    }
}
