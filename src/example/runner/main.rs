
use embed_wgpu_lib::*;



fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    env_logger::init();
    rt.block_on(dep::looper::looper::run());
}
