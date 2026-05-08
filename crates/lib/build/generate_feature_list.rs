pub(crate) struct Features(Vec<String>);

impl super::Task for Features {
    fn init() -> Self {
        let mut names = std::env::vars()
            .filter_map(|(key, _)| key.strip_prefix("CARGO_FEATURE_").map(str::to_owned))
            .collect::<Vec<_>>();

        names.sort_unstable();

        Self(names)
    }

    fn emit(self) {
        let out_dir = std::env::var("OUT_DIR").expect("environment variable `OUT_DIR` not found");

        let path = std::path::PathBuf::from(out_dir).join("features.rs");

        let contents = format!(
            "pub(crate) const ENABLED_FEATURES: &[&str] = &{:?};\n",
            self.0
        );

        std::fs::write(path, contents).expect("failed to write features.rs");
    }
}
