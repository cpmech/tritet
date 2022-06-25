use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

/// Creates a unique handle that can be shared between threads
pub(crate) fn generate_handle() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Allows to lock access to the c-code
pub(crate) static ACCESS_C_CODE: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));
