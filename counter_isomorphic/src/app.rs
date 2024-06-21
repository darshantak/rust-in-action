use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[cfg(feature = "ssr")]
use tracing::instrument;

#[cfg(feature="ssr")]
pub mod ssr_imports {
    pub use broadcaster::BroadcastChannel;
    pub use once_cell::sync::OnceCell;
    pub use std::sync::atomic::{AtomicI32,Ordering};
    // use lazy_static::*;
    
    pub static COUNT :AtomicI32 = AtomicI32::new(0);

    lazy_static::lazy_static! {
        pub static ref COUNT_CHANNEL: BroadcastChannel<i32> = BroadcastChannel::new();
    }

    static LOG_INIT: OnceCell<()> = OnceCell::new();

    pub fn init_logging() {
        LOG_INIT.get_or_init(|| {
            simple_logger::SimpleLogger::new().env().init().unwrap();
        });
    }
}

#[server]
#[cfg_attr(feature="ssr", instrument)]
pub async fn get_server_count()-> Result<i32,ServerFnError> {
    use ssr_imports::*;
    Ok(COUNT.load(Ordering::Relaxed))
}

#[server]
#[cfg_attr(feature="ssr", instrument)]
pub async fn adjust_server_count(delta:i32, msg:String) -> Result<i32, ServerFnError> {
    
}
