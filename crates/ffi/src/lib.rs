//! FFI bindings for initializing the Bomberman system.

use std::ffi::CStr;
use std::os::raw::c_char;

use engine::{SystemHandle, SystemInitializer, UnifiedConfig};
use tokio::runtime::Runtime;

/// Initialize the Bomberman system from a JSON configuration string.
/// # Safety
/// `config_json` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bomberman_init(config_json: *const c_char) -> *mut SystemHandle {
    let config_str = unsafe { CStr::from_ptr(config_json).to_string_lossy().to_string() };
    let config: UnifiedConfig =
        serde_json::from_str(&config_str).expect("Invalid configuration JSON");
    let mut initializer = SystemInitializer::new(config);
    let rt = Runtime::new().expect("runtime");
    let handle = rt
        .block_on(initializer.initialize())
        .expect("Failed to initialize system");
    Box::into_raw(Box::new(handle))
}

/// Shutdown the Bomberman system, freeing resources.
/// # Safety
/// `handle` must be a pointer obtained from [`bomberman_init`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bomberman_shutdown(handle: *mut SystemHandle) {
    if !handle.is_null() {
        unsafe {
            drop(Box::from_raw(handle));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_and_shutdown_via_ffi() {
        let json = r#"{
            "engine":{"width":5,"height":5,"tick_rate":60,"rules":{"max_players":4,"bomb_timer":3,"starting_lives":3}},
            "event_bus":{"buffer_size":10,"max_subscribers":10},
            "bots":[],
            "tournament":null,
            "ai":{},
            "rl":null,
            "bombs":{},
            "logging":{"level":"info"}
        }"#;
        let cstr = std::ffi::CString::new(json).unwrap();
        let ptr = unsafe { bomberman_init(cstr.as_ptr()) };
        assert!(!ptr.is_null());
        unsafe { bomberman_shutdown(ptr) };
    }
}
