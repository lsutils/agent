use log::error;
use std::panic;
use std::panic::PanicHookInfo;

fn hand(panic_info: &PanicHookInfo) {
    if cfg!(debug_assertions) {
        // cargo run .
        let err_message = format!("panic occurred {:?}", panic_info);
        error!("{}", err_message);
    } else {
        // cargo run --release
        let err_message = match panic_info.payload().downcast_ref::<&str>() {
            Some(&str) => {
                let err_message = format!(
                    "panic info: {:?}, occurred in {:?}",
                    str,
                    panic_info.location()
                );
                err_message
            }
            None => {
                let err_message = format!("panic occurred in {:?}", panic_info.location());
                err_message
            }
        };
        error!("{}", err_message);
    }
}
pub fn hook() {
    panic::set_hook(Box::new(|panic_info| {
        hand(panic_info);
    }));
}
