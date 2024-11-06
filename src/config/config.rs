use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(default = "UserConfig::standalone_default")]
pub struct UserConfig {
    name: String,
}

impl UserConfig {
    pub fn standalone_default() -> Self {
        let mut config = Self::default();
        config.set_standalone();
        config
    }

    fn set_standalone(&mut self) {
        self.name = "test".parse().unwrap();
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    pub controller_ips: Vec<String>,
    #[serde(skip)]
    pub controller_tls_port: u16,
    #[cfg(target_os = "linux")]
    pub pid_file: String,
    pub team_id: String,
}