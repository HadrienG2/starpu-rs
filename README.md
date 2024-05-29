# Rust Bindings for the StarPU runtime

[![MPL licensed](https://img.shields.io/badge/license-MPL-blue.svg)](./LICENSE)
<!--[![Package on
crates.io](https://img.shields.io/crates/v/starpu.svg)](https://crates.io/crates/starpu)
[![Documentation](https://docs.rs/starpu/badge.svg)](https://docs.rs/starpu/)-->
[![Continuous Integration](https://img.shields.io/github/actions/workflow/status/HadrienG2/starpu-rs/ci.yml?branch=master)](https://github.com/HadrienG2/starpu-rs/actions?query=workflow%3A%22Continuous+Integration%22)
<!--[![Code coverage](https://codecov.io/gh/HadrienG2/starpu-rs/graph/badge.svg?token=OYWLNUD9AI)](https://codecov.io/gh/HadrienG2/starpu-rs)
[![CII Best Practices Summary](https://img.shields.io/cii/summary/7876)](https://www.bestpractices.dev/en/projects/7876)-->
![Requires rustc
1.78.0+](https://img.shields.io/badge/rustc-1.78.0+-lightgray.svg)

## What is this?

Modern distributed computing platforms are complex and heterogeneous enough that
deciding which of the available hardware you should use to process a certain
task can easily become a challenge.

[StarPU](https://starpu.gitlabpages.inria.fr/) is a runtime system which helps
you to do so by letting you model your computation as a graph of asynchronous
tasks with multiple implementations, linked by data dependencies. The StarPU
runtime is then in charge of scheduling data transfers between available memory
resources (including across machines in a cluster, between
VRAM/DRAM/storage...), and scheduling execution over available execution
resources, with the aim of achieving optimal execution performance.

StarPU optimizes its scheduling decisions by dynamically measuring the
performance of the various task implementations as they run, and using these
measurements to build a performance model of how available hardware performs at
the task at hand. This model is, in turn, used to inform subsequent scheduling
decisions, including subsequent runs of the full computation.

Thanks to this modeling, StarPU is often able to match the performance of
computations where data transfers and scheduling decisions are implemented in
hand-coded logic. It can even outperform them in situations where the
hand-written algorithm fails to account for some regions of the parameter space
where a different implementation would be more optimal, or fails to leverage all
available hardware (e.g. only uses the GPU, leaving the CPU idle waiting for GPU
tasks to complete).

This repository aims to ultimately provide an idiomatic high-level Rust binding
for StarPU. At present time, however, it only provides the low-level C FFI
binding layer `starpu-sys`, which allows you to use StarPU via non-idiomatic "C
in Rust syntax" code. This is a necessary basic infrastructure, which the
high-level binding will later be able to build upon.

## Prerequisites

The crates within this repository have three basic dependencies:

1. A working [local installation of
   StarPU](https://files.inria.fr/starpu/doc/html_web_installation/BuildingAndInstallingStarPU.html).
   If you have installed it using a Linux distribution package, you will also
   need the associated `-dev` or `-devel` packages containing things like C API
   headers.
2. An implementation of the `pkg-config` command-line utility, such as
   [`pkgconf`](http://pkgconf.org/features.html), which is used to locate the
   various parts of the StarPU installation.
3. `libclang`, which is used in the process of translating the StarPU C headers
   to equivalent Rust declarations.

In addition, if StarPU is installed in a non-standard location, you will have to
make sure that your `PKG_CONFIG_PATH` is configured correctly (along the lines
of `${STARPU_PREFIX}/lib64/pkgconfig`, may be `lib` instead of `lib64` depending
on your Linux distribution).

If your StarPU installation has been built with OpenCL support (as is the case
for Debian/Ubuntu packages), you will also need to enable the
`starpu-sys/opencl` crate feature. It adds a dependency on the `cl-sys` crate
for the purpose of re-exporting its data types in the StarPU interface. You will
need to have a working OpenCL development environment for this to work.

## Usage

For now, read [the documentation of the StarPU C
API](https://starpu.gitlabpages.inria.fr/doc.html) and look for matching types
and functions in the `starpu-sys` crate.

Note that the API of the `starpu-sys` is automatically generated based on the C
headers of the version of StarPU that you have installed. Therefore, the set of
types and methods that you have available locally may differ slightly from the
public documentation at `docs.rs`, which is generated based on the headers of
the StarPU packages of the Ubuntu version that `docs.rs` uses.

You can use `cargo doc --open` and select `starpu_sys` on the sidebar to check
what exact API `starpu-sys` has on your system.

## StarPU API coverage

By design, `starpu-sys` will always cover the full C API of the version of
StarPU that you have installed, except for concepts which cannot be readily
translated to Rust like macros and inline functions.

On the other hand, once it will be a thing, `starpu` will only cover the part of
the StarPU API for which a high-level interface has been written. That part will
be documented here.

## License

This project uses the MPLv2 license, please see the
[LICENSE](https://github.com/hadrieng2/starpu-rs/blob/master/LICENSE) file for
more information.
