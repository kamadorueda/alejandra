use alejandra::config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ConfigOptions {}

impl From<ConfigOptions> for Config {
    fn from(_config_options: ConfigOptions) -> Config {
        let config = Config::default();

        config
    }
}
