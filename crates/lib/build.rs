#[path = "build/mod.rs"]
mod build;

use {build::Task, build::export_build_target::BuildTarget, build::generate_feature_list::Features};

pub(crate) const TASKS: &[fn()] = &[BuildTarget::run, Features::run];

fn main() {
    for task in TASKS {
        task();
    }
}
