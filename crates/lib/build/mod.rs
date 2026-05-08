pub(crate) mod export_build_target;
pub(crate) mod generate_feature_list;

pub(crate) trait Task: Sized {
    fn init() -> Self;
    fn emit(self);

    fn run() { Self::init().emit(); }
}
