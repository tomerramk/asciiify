fn main() {
    // napi_build::setup() links libnode.dll on Windows, which may not be
    // available during `cargo check`. Only run the full setup when actually
    // building the cdylib (i.e. CARGO_CFG_NAPI_RS_CLI_VERSION is set by
    // the @napi-rs/cli toolchain).
    if std::env::var("CARGO_CFG_NAPI_RS_CLI_VERSION").is_ok() {
        #[cfg(windows)]
        {
            // Emit the rerun-if-env-changed markers napi_build normally emits.
            println!("cargo:rerun-if-env-changed=DEBUG_GENERATED_CODE");
            println!("cargo:rerun-if-env-changed=TYPE_DEF_TMP_PATH");
            println!("cargo:rerun-if-env-changed=CARGO_CFG_NAPI_RS_CLI_VERSION");
            println!("cargo::rerun-if-env-changed=NAPI_DEBUG_GENERATED_CODE");
            println!("cargo::rerun-if-env-changed=NAPI_TYPE_DEF_TMP_FOLDER");

            // napi-build 2.x searches for libnode.dll, but standard Node.js
            // installations ship node.lib instead.  Find it ourselves.
            if let Ok(dir) = std::env::var("NAPI_NODE_LIB_DIR") {
                // If the CLI already set the dir, honour it.
                println!("cargo:rustc-link-search=native={}", dir);
                println!("cargo:rustc-link-lib=libnode");
            } else if let Some(dir) = find_node_dir() {
                println!("cargo:rustc-link-search=native={}", dir);
                println!("cargo:rustc-link-lib=node");
            } else {
                // Nothing found — fall back to napi_build (will emit a clear error).
                napi_build::setup();
            }
        }
        #[cfg(not(windows))]
        napi_build::setup();
    }
}

/// Search PATH for a directory that contains both `node.exe` and `node.lib`.
#[cfg(windows)]
fn find_node_dir() -> Option<String> {
    let path_var = std::env::var("PATH").ok()?;
    for dir in std::env::split_paths(&path_var) {
        if dir.join("node.exe").exists() && dir.join("node.lib").exists() {
            return Some(dir.to_string_lossy().into_owned());
        }
    }
    None
}
