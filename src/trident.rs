#![allow(dead_code)]

use std::fmt;
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use crate::config::{Config, UserConfig};

#[derive(Debug, Default)]
pub struct ChangedConfig {
    pub user_config: UserConfig,
    pub blacklist: Vec<u64>,
}

#[derive(Debug)]
pub enum State {
    Running,
    ConfigChanged(ChangedConfig),
    Terminated,
}

impl State {
    fn unwrap_config(self) -> ChangedConfig {
        match self {
            Self::ConfigChanged(c) => c,
            _ => panic!("{:?} not config type", &self),
        }
    }
}

pub struct VersionInfo {
    pub name: &'static str,
    // pub branch: &'static str,
    // pub commit_id: &'static str,
    // pub rev_count: &'static str,
    // pub compiler: &'static str,
    // pub compile_time: &'static str,
    // pub revision: &'static str,
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}",
            match self.name {
                "deepflow-agent-ce" => "deepflow-agent community edition",
                "deepflow-agent-ee" => "deepflow-agent enterprise edition",
                _ => panic!("{:?} unknown deepflow-agent edition", self.name),
            },
        )
    }
}

pub type AgentState = Arc<(Mutex<State>, Condvar)>;

#[derive(Clone, Default, Copy, PartialEq, Eq, Debug)]
pub enum RunningMode {
    #[default]
    Managed,
    Standalone,
}

pub struct Trident {
    state: AgentState,
    handle: Option<JoinHandle<()>>,
}

impl Trident {
    pub fn start<P: AsRef<Path>>(
        config_path: P,
        version_info: &'static VersionInfo,
        agent_mode: RunningMode,
        sidecar_mode: bool,
        cgroups_disabled: bool,
    ) {
        let config = Config {
            controller_ips: vec![],
            controller_port: 0,
            controller_tls_port: 0,
            controller_cert_file_prefix: "".to_string(),
            log_file: "".to_string(),
            kubernetes_cluster_id: "".to_string(),
            kubernetes_cluster_name: None,
            vtap_group_id_request: "".to_string(),
            controller_domain_name: vec![],
            override_os_hostname: None,
            async_worker_thread_number: 0,
            team_id: "".to_string(),
            cgroups_disabled,
            new_rpc: false,
        };
        // #[cfg(target_os = "linux")]
        !config.pid_file.is_empty();
    }
}













