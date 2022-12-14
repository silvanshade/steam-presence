use serde::{Deserialize, Serialize};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    DirectoriesBaseDirsNew,
    SerdeJsonFromStr { source: serde_json::Error },
    SerdeJsonFromValue { source: serde_json::Error },
    SerdeJsonToVec { source: serde_json::Error },
    StdFsCreateDirAll { source: std::io::Error },
    StdFsMetadata { source: std::io::Error },
    TokioFsOpenOptions { source: std::io::Error },
    TokioIoReadToString { source: std::io::Error },
    StdFsSyncAll { source: std::io::Error },
    TokioIoWriteAll { source: std::io::Error },
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub services: Services,
    pub activity: Activity,
    pub games: Games,
}

impl Config {
    const FILE_NAME: &str = "config.json";

    fn file_base() -> Result<std::path::PathBuf, Error> {
        let base = directories::BaseDirs::new().context(DirectoriesBaseDirsNewSnafu)?;
        let mut path = base.config_dir().to_path_buf();
        path.push("game-presence");
        Ok(path)
    }

    fn file_base_create() -> Result<(), Error> {
        let base = Self::file_base()?;
        std::fs::create_dir_all(base).context(StdFsCreateDirAllSnafu)?;
        Ok(())
    }

    fn file_path() -> Result<std::path::PathBuf, Error> {
        let mut path = Self::file_base()?;
        path.push(Self::FILE_NAME);
        Ok(path)
    }

    pub async fn read() -> Result<Self, Error> {
        use tokio::io::AsyncReadExt;
        Self::file_base_create()?;
        let path = Self::file_path()?;
        let path = path.as_path();
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(path)
            .await
            .context(TokioFsOpenOptionsSnafu)?;
        let mut json = String::new();
        file.read_to_string(&mut json).await.context(TokioIoReadToStringSnafu)?;
        let value = serde_json::from_str::<Self>(&json);
        let config = match value {
            Err(_) => {
                let config = Self::default();
                config.write().await?;
                config
            },
            Ok(config) => config,
        };
        Ok(config)
    }

    pub async fn write(&self) -> Result<(), Error> {
        use tokio::io::AsyncWriteExt;
        Self::file_base_create()?;
        let path = Self::file_path()?;
        let path = path.as_path();
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .await
            .context(TokioFsOpenOptionsSnafu)?;
        let json = serde_json::to_vec_pretty(self).context(SerdeJsonToVecSnafu)?;
        file.write_all(&json).await.context(TokioIoWriteAllSnafu)?;
        file.sync_all().await.context(StdFsSyncAllSnafu)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Services {
    pub nintendo: self::service::Nintendo,
    pub playstation: self::service::Playstation,
    pub steam: self::service::Steam,
    pub twitch: self::service::Twitch,
    pub xbox: self::service::Xbox,
}

pub mod service {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Nintendo {
        pub disclaimer_acknowledged: bool,
        pub enabled: bool,
        pub assets_priorities: Vec<super::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::nintendo::Data>,
    }

    impl Default for self::Nintendo {
        fn default() -> Self {
            let disclaimer_acknowledged = bool::default();
            let enabled = bool::default();
            let assets_priorities = vec![super::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                disclaimer_acknowledged,
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    pub mod nintendo {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub username: Option<String>,
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Playstation {
        pub enabled: bool,
        pub assets_priorities: Vec<super::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::playstation::Data>,
    }

    impl Default for self::Playstation {
        fn default() -> Self {
            let enabled = bool::default();
            let assets_priorities = vec![super::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    pub mod playstation {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub username: Option<String>,
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Steam {
        pub enabled: bool,
        pub assets_priorities: Vec<super::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::steam::Data>,
    }

    impl Default for self::Steam {
        fn default() -> Self {
            let enabled = bool::default();
            let assets_priorities = vec![super::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    pub mod steam {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub id: String,
            pub key: String,
            pub username: String,
        }
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Twitch {
        pub enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::twitch::Data>,
    }

    pub mod twitch {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub username: String,
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Xbox {
        pub enabled: bool,
        pub assets_priorities: Vec<super::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::xbox::Data>,
    }

    impl Default for self::Xbox {
        fn default() -> Self {
            let enabled = bool::default();
            let assets_priorities = vec![super::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    pub mod xbox {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub username: Option<String>,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetSourceEntry {
    #[default]
    Native,
    Twitch,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub polling_active: bool,
    pub discord_display_presence: bool,
    pub games_require_whitelisting: bool,
    pub service_priorities: Vec<ServicePrioritiesEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServicePrioritiesEntry {
    Nintendo,
    Playstation,
    Steam,
    Xbox,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {}
