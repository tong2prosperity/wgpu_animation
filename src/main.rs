use crate::dep::looper::looper;

pub mod dep;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    env_logger::init();
    rt.block_on(looper::run());
}
