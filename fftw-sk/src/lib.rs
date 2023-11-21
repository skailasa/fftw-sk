extern crate fftw_sk_sys as ffi;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    /// Mutex for FFTW call.
    ///
    /// This mutex is necessary because most of calls in FFTW are not thread-safe.
    /// See the [original document](http://www.fftw.org/fftw3_doc/Thread-safety.html) for detail
    pub static ref FFTW_MUTEX: Mutex<()> = Mutex::new(());
}

/// Exclusive call of FFTW interface.
macro_rules! excall {
    ($call:expr) => {{
        let _lock = $crate::FFTW_MUTEX.lock().expect("Cannot get lock");
        unsafe { $call }
    }};
} // excall!

pub mod array;
pub mod error;
pub mod plan;
pub mod types;