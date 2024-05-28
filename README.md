# Low-level bindings to StarPU

This Rust crate allows you to use [StarPU](https://starpu.gitlabpages.inria.fr/)
in Rust almost like a C program would.

Needless to say, the resulting Rust code is not very idiomatic (almost every
line of code is unsafe), but the plan is to eventually provide higher-level safe
abstractions in a separate `starpu` crate, once I have fewer
[things](https://github.com/HadrienG2/hwlocality)
[in](https://github.com/rayon-rs/rayon/issues/319#issuecomment-1783731222)
[my](https://indico.in2p3.fr/event/30939/)
[backlog](https://gitlab.in2p3.fr/baoradio/tacq).

To build this crate, you will need a local installation of libclang and StarPU,
and a correctly configured `PKG_CONFIG_PATH` (along the lines of
`${STARPU_PREFIX}/lib64/pkgconfig`).
