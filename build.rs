use bindgen::EnumVariation;
use std::{env, path::PathBuf, sync::OnceLock};

fn main() {
    // We don't need StarPU on docs.rs since it only builds the docs
    // FIXME: Actually, we do need it since we use bindgen. Find out how we
    //        could handle docs.rs correctly.
    if std::env::var("DOCS_RS").is_err() {
        setup_starpu();
    }
}

/// Configure the StarPU dependency
fn setup_starpu() {
    // Select desired StarPU version
    // FIXME: Stop hardcoding, select the right version based on features as
    //        done in hwlocality_sys.
    let required_version = "1.4.0";

    // Find the library using pkg-config
    let library = find_starpu(required_version);

    // Generate the Rust bindings
    let bindings = generate_bindings(&library);

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

/// Use pkg-config to locate and use a certain StarPu release
fn find_starpu(required_version: &str) -> pkg_config::Library {
    // Initialize pkg-config
    let mut config = pkg_config::Config::new();

    // Specify the required version range
    let mut version_components = required_version.split('.');
    let major_version = version_components.next().expect("No major version");
    let minor_version = version_components.next().expect("No minor version");
    let next_minor_version = minor_version
        .parse::<usize>()
        .expect("Minor version isn't numeric")
        + 1;
    let next_unsupported_version = format!("{major_version}.{next_minor_version}.0");
    config.range_version(required_version..&next_unsupported_version);

    // Run pkg-config
    let lib = config
        // FIXME: Make static builds work outside macos (target_os() !=
        //        "macos"), as done in hwlocality
        .statik(false)
        .probe(&format!("starpu-{major_version}.{minor_version}"))
        .expect("Could not find a suitable version of StarPU");

    // As it turns-out, pkg-config does not correctly set up the RPATHs for the
    // transitive dependencies of starpu itself in static builds. Fix that.
    if target_family().split(',').any(|family| family == "unix") {
        for link_path in &lib.link_paths {
            println!(
                "cargo:rustc-link-arg=-Wl,-rpath,{}",
                link_path
                    .to_str()
                    .expect("Link path is not an UTF-8 string")
            );
        }
    }

    // Forward pkg-config output for futher consumption
    lib
}

/// Generate StarPU bindings using bindgen
fn generate_bindings(library: &pkg_config::Library) -> bindgen::Bindings {
    bindgen::Builder::default()
        .allowlist_file(".*/starpu/.*")
        .allowlist_recursively(false)
        .allowlist_type(".*va_list.*|drand48_data")
        .array_pointers_in_arguments(true)
        .clang_args(
            library
                .include_paths
                .iter()
                .flat_map(|include_path| {
                    ["-isystem".to_string(), include_path.display().to_string()]
                })
                .chain(std::iter::once(
                    "-fretain-comments-from-system-headers".to_string(),
                ))
                .collect::<Vec<_>>(),
        )
        .constified_enum_module("starpu_task_status")
        .default_enum_style(EnumVariation::Consts)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .generate_cstr(true)
        .header("wrapper.h")
        .impl_debug(true)
        .impl_partialeq(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate StarPU bindings")
}

/// Cross-compilation friendly alternative to `cfg!(target_os)`
#[allow(unused)]
fn target_os() -> &'static str {
    static TARGET_OS: OnceLock<Box<str>> = OnceLock::new();
    TARGET_OS
        .get_or_init(|| {
            std::env::var("CARGO_CFG_TARGET_OS")
                .expect("Cargo should tell us what the target OS is")
                .into_boxed_str()
        })
        .as_ref()
}

/// Cross-compilation friendly alternative to `cfg!(target_family)`
fn target_family() -> &'static str {
    static TARGET_FAMILY: OnceLock<Box<str>> = OnceLock::new();
    TARGET_FAMILY
        .get_or_init(|| {
            std::env::var("CARGO_CFG_TARGET_FAMILY")
                .expect("Cargo should tell us what the target family is")
                .into_boxed_str()
        })
        .as_ref()
}
