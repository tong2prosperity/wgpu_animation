
use embed_wgpu_lib::*;



fn main() {
    env_logger::Builder::from_default_env()
        .filter(Some("wgpu_core"), log::LevelFilter::Warn)
        .filter(Some("wgpu_hal"), log::LevelFilter::Warn)
        .filter_level(log::LevelFilter::Info)
        .init();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(dep::looper::looper::run());
}
