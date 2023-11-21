#![allow(non_camel_case_types)]

#[cfg(feature = "source")]
extern crate fftw_sk_src as ffi;

use libc::FILE;
// pub use num_complex::Complex32 as fftwf_complex;
// pub use num_complex::Complex64 as fftw_complex;
pub type fftwf_complex = [f32; 2];
pub type fftw_complex = [f64; 2];

#[cfg_attr(feature = "system", link(name = "fftw3"))]
extern "C" {}
#[cfg_attr(feature = "system", link(name = "fftw3f"))]
extern "C" {}

include!("fftw.rs");