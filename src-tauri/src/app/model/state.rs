use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {}

#[derive(Debug)]
pub struct State {
    pub profiles: Vec<Profile>,
}

#[derive(Debug)]
pub struct Profile {
    pub services: Services,
    pub activity: Activity,
    pub games: Games,
}

#[derive(Debug)]
pub struct Services {
    pub nintendo: Option<self::service::Nintendo>,
    pub playstation: Option<self::service::Playstation>,
    pub steam: Option<self::service::Steam>,
    pub xbox: Option<self::service::Xbox>,
}

pub mod service {
    #[derive(Debug)]
    pub struct Nintendo {
        pub enabled: bool,
    }

    impl TryFrom<crate::app::data::config::service::Nintendo> for self::Nintendo {
        type Error = super::Error;

        fn try_from(nintendo: crate::app::data::config::service::Nintendo) -> Result<Self, Self::Error> {
            todo!()
        }
    }

    #[derive(Debug)]
    pub struct Playstation {
        pub enabled: bool,
    }

    impl TryFrom<crate::app::data::config::service::Playstation> for self::Playstation {
        type Error = super::Error;

        fn try_from(nintendo: crate::app::data::config::service::Playstation) -> Result<Self, Self::Error> {
            todo!()
        }
    }

    #[derive(Debug)]
    pub struct Steam {
        pub enabled: bool,
        pub id: String,
        pub key: String,
    }

    impl TryFrom<crate::app::data::config::service::Steam> for self::Steam {
        type Error = super::Error;

        fn try_from(nintendo: crate::app::data::config::service::Steam) -> Result<Self, Self::Error> {
            todo!()
        }
    }

    #[derive(Debug)]
    pub struct Xbox {
        pub enabled: bool,
    }

    impl TryFrom<crate::app::data::config::service::Xbox> for self::Xbox {
        type Error = super::Error;

        fn try_from(nintendo: crate::app::data::config::service::Xbox) -> Result<Self, Self::Error> {
            todo!()
        }
    }
}

#[derive(Debug)]
pub struct Activity {
    pub discord_display_presence: bool,
    pub twitch_assets_enabled: bool,
    pub games_require_whitelisting: bool,
}

#[derive(Debug)]
pub struct Games {}

impl TryFrom<crate::app::data::Config> for State {
    type Error = Error;

    fn try_from(config: crate::app::data::Config) -> Result<Self, Self::Error> {
        let profiles = config
            .profiles
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        Ok(Self { profiles })
    }
}

impl TryFrom<crate::app::data::config::Profile> for self::Profile {
    type Error = Error;

    fn try_from(profile: crate::app::data::config::Profile) -> Result<Self, Self::Error> {
        let services = profile.services.try_into()?;
        let activity = profile.activity.try_into()?;
        let games = profile.games.try_into()?;
        Ok(Self {
            services,
            activity,
            games,
        })
    }
}

impl TryFrom<crate::app::data::config::Services> for self::Services {
    type Error = Error;

    fn try_from(services: crate::app::data::config::Services) -> Result<Self, Self::Error> {
        let nintendo = services.nintendo.map(TryInto::try_into).transpose()?;
        let playstation = services.playstation.map(TryInto::try_into).transpose()?;
        let steam = services.steam.map(TryInto::try_into).transpose()?;
        let xbox = services.xbox.map(TryInto::try_into).transpose()?;
        Ok(Self {
            nintendo,
            playstation,
            steam,
            xbox,
        })
    }
}

impl TryFrom<crate::app::data::config::Activity> for self::Activity {
    type Error = Error;

    fn try_from(activity: crate::app::data::config::Activity) -> Result<Self, Self::Error> {
        let discord_display_presence = activity.discord_display_presence;
        let twitch_assets_enabled = activity.twitch_assets_enabled;
        let games_require_whitelisting = activity.games_require_whitelisting;
        Ok(Self {
            discord_display_presence,
            twitch_assets_enabled,
            games_require_whitelisting,
        })
    }
}

impl TryFrom<crate::app::data::config::Games> for self::Games {
    type Error = Error;

    fn try_from(_: crate::app::data::config::Games) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
