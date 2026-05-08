pub(crate) struct BuildTarget(String);

impl super::Task for BuildTarget {
    fn init() -> Self { Self(std::env::var("TARGET").expect("environment variable `TARGET` not found")) }

    fn emit(self) {
        println!("cargo:rustc-env=BUILD_TARGET={}", self.0);
    }
}
