/// StarPU bindings
#[allow(
    improper_ctypes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_html_tags,
    rustdoc::bare_urls
)]
mod starpu {
    use hwlocality_sys::*;
    use libc::*;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use starpu::*;

/// Re-export libc and hwloc definitions
pub use hwlocality_sys;
pub use libc;
