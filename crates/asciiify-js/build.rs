fn main() {
    // napi_build::setup() links libnode.dll on Windows, which may not be
    // available during `cargo check`. Only run the full setup when actually
    // building the cdylib (i.e. CARGO_CFG_NAPI_RS_CLI_VERSION is set by
    // the @napi-rs/cli toolchain).
    if std::env::var("CARGO_CFG_NAPI_RS_CLI_VERSION").is_ok() {
        napi_build::setup();
    }
}
