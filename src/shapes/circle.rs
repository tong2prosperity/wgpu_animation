use lyon::algorithms::rounded_polygon;
use lyon::tessellation::{self, BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers};
use lyon::math::{point, Point};
use lyon::path::{Path, PathEvent, Polygon, NO_ATTRIBUTES};



#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct IVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub fn generate_circle(radius: f32) -> VertexBuffers<IVertex, u16> {
    let mut path_builder = Path::builder();
    path_builder.add_circle(point(0.0, 0.0), radius, lyon::path::Winding::Negative);
    let path = path_builder.build();

    let mut buffers: VertexBuffers<IVertex, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();

    tessellator.tessellate_path(
        &path,
        &FillOptions::tolerance(0.02),
        &mut BuffersBuilder::new(&mut buffers, |vb: FillVertex| {
            let pos = vb.position().to_array();
            IVertex{
                position: [pos[0], pos[1], 0.0],
                color: [1.0, 0.0, 0.0],
            }
        }),
    ).unwrap();

    for ref vertex in &buffers.vertices {
        println!("pos ({},{})", vertex.position[0], vertex.position[1])
    }

    for ref ind in & buffers.indices {
        println!("index {}", ind);
    }

    buffers
}

pub fn generate_arrow(radius: f32, length: f32) -> VertexBuffers<IVertex, u16> {
    let arrow_points = [
        point(-1.0, -0.3),
        point(0.0, -0.3),
        point(0.0, -1.0),
        point(1.5, 0.0),
        point(0.0, 1.0),
        point(0.0, 0.3),
        point(-1.0, 0.3),
    ];

    let arrow_polygon = Polygon {
        points: &arrow_points,
        closed: true,
    };

    let mut buffers: VertexBuffers<IVertex, u16> = VertexBuffers::new();


    let mut fill_tess = FillTessellator::new();


    let mut arrow_builder = Path::builder();
    rounded_polygon::add_rounded_polygon(&mut arrow_builder, arrow_polygon, 0.2, NO_ATTRIBUTES);
    let arrow_path = arrow_builder.build();

    fill_tess
        .tessellate_path(
            &arrow_path,
            &FillOptions::tolerance(0.02),
            &mut BuffersBuilder::new(&mut buffers, |v: FillVertex| {
                let pos = v.position().to_array();
                IVertex{
                    position: [pos[0], pos[1], 0.0],
                    color: [0.0, 0.0, 0.0],
                }
            }),
        )
        .unwrap();

    buffers

}
