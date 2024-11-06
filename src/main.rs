mod config;
mod panic_hook;
mod proto;
mod trident;

use std::path::Path;
use std::sync::{Arc, Condvar, Mutex};
use clap::{ArgAction, Parser};
use log::LevelFilter;

#[derive(Parser, Debug)]
struct Opts {
    /// Specify config file location
    #[clap(
        short = 'f',
        visible_short_alias = 'c',
        long,
        default_value = "/etc/deepflow-agent.yaml"
    )]
    config_file: String,

    /// Enable standalone mode, default config path is /etc/deepflow-agent-standalone.yaml
    #[clap(long)]
    standalone: bool,

    /// Display the version
    #[clap(short, long, action = ArgAction::SetTrue)]
    version: bool,
}
#[cfg(any(target_os = "macos", target_os = "linux", target_os = "android"))]
use signal_hook::{consts::TERM_SIGNALS, iterator::Signals};

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "android"))]
fn wait_on_signals() {
    let mut signals = Signals::new(TERM_SIGNALS).unwrap();
    signals.forever().next();
    signals.handle().close();
}

#[cfg(windows)]
fn wait_on_signals() {}

const VERSION_INFO: &'static trident::VersionInfo = &trident::VersionInfo {
    name: env!("AGENT_NAME"), // 在编译时将环境变量的值嵌入到程序中。
};

// fn main() -> anyhow::Result<()> {
//     let _ = env_logger::builder()
//         .is_test(true)
//         .filter(None, LevelFilter::Debug)
//         .try_init();
//
//     panic_hook::hook();
//
//     let opts = Opts::parse();
//     println!("{:?}", opts);
//
//     if opts.version {
//         println!("{}", VERSION_INFO);
//         return Ok(());
//     }
//
//     let mut t = trident::Trident::start(
//         &Path::new(&opts.config_file),
//         VERSION_INFO,
//         if opts.standalone {
//             trident::RunningMode::Standalone
//         } else {
//             trident::RunningMode::Managed
//         },
//     )?;
//
//     wait_on_signals();
//     t.stop();
//     Ok(())
// }
//
#[derive(Debug)]
pub enum State {
    Running,
    Terminated,
}
fn main() {
    pub type AgentState = Arc<(Mutex<State>, Condvar)>;

    let x = AgentState::new((Mutex::new(State::Running), Default::default()));





}