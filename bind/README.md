# rebound-bind

Low-level Rust FFI bindings for the [REBOUND](https://github.com/hannorein/rebound) N-body simulation C library.

## Requirements

Building this crate requires:

- A C compiler
- `libclang` and Clang headers for `bindgen`

Some optional features require additional system libraries.

## Features

No features are enabled by default.

Available features:

- `server`: enable REBOUND server support
- `opengl`: enable OpenGL display support; requires GLFW
- `openmp`: enable OpenMP support
- `openmp-clang`: enable OpenMP support for Clang/libomp setups
- `mpi`: enable MPI support; requires an MPI compiler and headers
- `fftw`: enable FFTW support; requires FFTW3
- `avx512`: enable AVX512 support
- `quadrupole`: enable quadrupole support
- `profiling`: enable profiling support

Example:

```sh
cargo add rebound-bind --features server
```

## License

This crate is licensed under GPL-3.0 and includes vendored REBOUND sources under the same license family. See `rebound/LICENSE` for the upstream license text.
