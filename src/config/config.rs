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
    pub controller_port: u16,
    pub controller_tls_port: u16,
    pub controller_cert_file_prefix: String,
    pub log_file: String,
    pub kubernetes_cluster_id: String,
    pub kubernetes_cluster_name: Option<String>,
    pub vtap_group_id_request: String,
    pub controller_domain_name: Vec<String>,
    #[serde(skip)]
    // pub agent_mode: RunningMode,
    pub override_os_hostname: Option<String>,
    pub async_worker_thread_number: u16,
    // pub agent_unique_identifier: AgentIdType,
    #[cfg(target_os = "linux")]
    pub pid_file: String,
    pub team_id: String,
    pub cgroups_disabled: bool,
    pub new_rpc: bool,
}