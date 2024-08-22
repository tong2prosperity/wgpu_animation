use lyon::tessellation::{
    self, FillOptions, FillTessellator, VertexBuffers,
};
use lyon::math::{point, Point};
use lyon::path::{Path, PathEvent};

pub fn generate_circle(radius: f32) -> VertexBuffers<Point, u16> {
    let mut path_builder = Path::builder();
    path_builder.add_circle(point(0.0, 0.0), radius, lyon::path::Winding::Positive);
    let path = path_builder.build();

    let mut buffers: VertexBuffers<Point, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();

    tessellator.tessellate_path(
        &path,
        &FillOptions::default(),
        &mut buffers,
    ).unwrap();

    buffers
}
