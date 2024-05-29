# starpu-sys: Low-level bindings to StarPU

[![MPL licensed](https://img.shields.io/badge/license-MPL-blue.svg)](../LICENSE)
[![Package on crates.io](https://img.shields.io/crates/v/starpu-sys.svg)](https://crates.io/crates/starpu-sys)
[![Documentation](https://docs.rs/starpu-sys/badge.svg)](https://docs.rs/starpu-sys/)
[![Continuous Integration](https://img.shields.io/github/actions/workflow/status/HadrienG2/starpu-rs/ci.yml?branch=master)](https://github.com/HadrienG2/starpu-rs/actions?query=workflow%3A%22Continuous+Integration%22)
<!--[![CII Best Practices Summary](https://img.shields.io/cii/summary/7876)](https://www.bestpractices.dev/en/projects/7876)-->![Requires rustc
1.78.0+](https://img.shields.io/badge/rustc-1.78.0+-lightgray.svg)

This crate contains unsafe Rust bindings to the C API of
[StarPU](https://starpu.gitlabpages.inria.fr/index.html).

Using these bindings directly is basically writing C in Rust syntax, which is
neither idiomatic nor safe. But the intent is to later build an idiomatic safe
Rust API on top of these bindings, in a separate crate that will be called
`starpu` if the name hasn't been taken by then.

Installation instructions and a project overview can be found in [the source
repository's toplevel
README](https://github.com/hadrieng2/starpu-rs/blob/master/README.md).
