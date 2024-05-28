use bindgen::EnumVariation;
use std::{env, path::PathBuf, sync::OnceLock};

fn main() {
    // We don't need hwloc on docs.rs since it only builds the docs
    if std::env::var("DOCS_RS").is_err() {
        setup_hwloc();
    }
}

const DEFAULT_VERSION: &str = "1.4.0";

/// Configure the hwloc dependency
fn setup_hwloc() {
    // Select desired StarPU version
    // FIXME: Stop hardcoding, select the right version based on features as in
    //        hwlocality.
    let required_version = DEFAULT_VERSION;

    // Find the library using pkg-config
    let library = find_hwloc(required_version);

    // Generate the bindings (TODO: Adapt search path if pkg_config isn't
    // enough).
    let bindings = bindgen::Builder::default()
        .clang_args(
            dbg!(library.include_paths.iter().flat_map(|include_path| [
                "-isystem".to_string(),
                include_path.display().to_string()
            ]))
            .collect::<Vec<_>>(),
        )
        .default_enum_style(EnumVariation::ModuleConsts)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

/// Use pkg-config to locate and use a certain hwloc release
fn find_hwloc(required_version: &str) -> pkg_config::Library {
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
        // FIXME: Make static builds work outsice macos (target_os() != "macos")
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
