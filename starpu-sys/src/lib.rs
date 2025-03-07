#![doc = include_str!("../README.md")]

/// StarPU bindings
#[allow(
    clippy::missing_safety_doc,
    clippy::ptr_offset_with_cast,
    clippy::too_many_arguments,
    clippy::useless_transmute,
    improper_ctypes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_html_tags,
    rustdoc::bare_urls
)]
mod bindgen {
    #[cfg(feature = "opencl")]
    use cl_sys::*;
    use hwlocality_sys::*;
    use libc::*;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use bindgen::*;

// Re-export types from other libraries used in the StarPU bindings

/// OpenCL types (only available if the "opencl" feature is enabled)
#[cfg(feature = "opencl")]
pub use cl_sys;

/// hwloc types
pub use hwlocality_sys;

/// libc types
pub use libc;
