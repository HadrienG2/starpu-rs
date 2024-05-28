use bindgen::EnumVariation;
use std::{env, path::PathBuf};

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
    // Find the library using pkg-config
    let library = find_starpu();

    // Generate the Rust bindings
    let bindings = generate_bindings(&library);

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

/// Use pkg-config to locate and use a certain StarPu release
fn find_starpu() -> pkg_config::Library {
    pkg_config::Config::new()
        .probe("libstarpu")
        .expect("Could not find StarPU")
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
