use std::{env, fs, path::Path};

// Workaround: Cargo only exposes certain build info (like TARGET and features)
// to build scripts, so we forward it into generated code / env vars for runtime use.

fn main() {
    // Expose target triple
    let target = env::var("TARGET").expect("TARGET not set by Cargo");
    println!("cargo:rustc-env=BUILD_TARGET={target}");

    // Collect enabled features
    let mut features: Vec<String> = env::vars()
        .filter_map(|(k, _)| k.strip_prefix("CARGO_FEATURE_").map(str::to_owned))
        .collect();

    features.sort();

    // Generate features.rs
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set by Cargo");
    let path = Path::new(&out_dir).join("features.rs");

    fs::write(
        path,
        format!(
            "pub(crate) const ENABLED_FEATURES: &[&str] = &{:?};",
            features
        ),
    )
    .expect("failed to write features.rs");
}
