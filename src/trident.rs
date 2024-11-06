#![allow(dead_code)]

use std::{fmt, mem};
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use log::info;
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
    ) {
        let config = Config {
            controller_ips: vec![],
            controller_tls_port: 0,
            team_id: "".to_string(),
        };
        // #[cfg(target_os = "linux")]
        !config.pid_file.is_empty();
    }


    pub fn stop(&mut self) {
        info!("Gracefully stopping");
        let (state, cond) = &*self.state;

        let mut state_guard = state.lock().unwrap();
        *state_guard = State::Terminated;
        cond.notify_one();
        mem::drop(state_guard);
        self.handle.take().unwrap().join().unwrap();
        info!("Gracefully stopped");
    }
}













