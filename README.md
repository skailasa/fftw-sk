# Rust FFTW

This is a modified version of the FFTW bindings in Rust originally released [here](https://github.com/rust-math/fftw).

The major modifications are a restriction to R2C/C2R transforms, the implementation of Send for Plans to allow multi-threaded sharing of FFT plans, as well as the removal of the reliance on the `Complex<T>` type from the `num_complex` trait for performance reasons, as initialising a vector of structs becomes onerous when computing large FFTs. Instead complex numbers are handled as in C/C++ with an array `[T; 2]`.

Otherwise the interface remains the same as in the original crate.