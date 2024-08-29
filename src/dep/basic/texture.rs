use super::*;


pub fn create_texture_view(device: &Device, config: &SurfaceConfiguration, sample_count: u32) -> TextureView {
    let multisampled_texture_extent = wgpu::Extent3d {
        width: config.width,
        height: config.height,
        depth_or_array_layers: 1,
    };

    let multisampled_texture_desc = wgpu::TextureDescriptor {
        size: multisampled_texture_extent,
        mip_level_count: 1,
        sample_count: sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: config.format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        label: Some("Multisampled texture"),
        view_formats: &config.view_formats,
    };

    let multisampled_texture = device.create_texture(&multisampled_texture_desc);
    let multisampled_view = multisampled_texture.create_view(&wgpu::TextureViewDescriptor::default());
    multisampled_view
}