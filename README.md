# Low-level Rust bindings to StarPU

This Rust crate allows you to use [StarPU](https://starpu.gitlabpages.inria.fr/)
in Rust almost like a C program would.

Needless to say, the resulting Rust code is not very idiomatic (almost every
line of code is unsafe), but the plan is to eventually provide higher-level safe
abstractions in a separate `starpu` crate, once I have fewer
[things](https://github.com/HadrienG2/hwlocality)
[in](https://github.com/rayon-rs/rayon/issues/319#issuecomment-1783731222)
[my](https://indico.in2p3.fr/event/30939/)
[backlog](https://gitlab.in2p3.fr/baoradio/tacq).

To build this crate, you will need to install libclang, StarPU, and an
implementation of `pkg-config`. If StarPU is not installed in a standard library
path, you will also want to make sure that your `PKG_CONFIG_PATH` is configured
correctly (along the lines of `${STARPU_PREFIX}/lib64/pkgconfig`, may be `lib`
instead of `lib64` depending on your Linux distribution).

If your StarPU installation has been built with OpenCL support, you will need to
enable the `opencl` feature. It adds a dependency on the `cl-sys` crate for the
purpose of re-exporting its data types in the StarPU interface. You will need to
have a working OpenCL development environment in this case.
