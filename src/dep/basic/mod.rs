pub mod state;
pub mod structure;
pub(crate) mod resources;
pub mod shapes;
pub mod action;
mod texture;
mod projection;
mod instance;

use resources::*;
use action::*;
use shapes::*;
use texture::*;


use wgpu::{BindGroupEntry, Device, SurfaceConfiguration, TextureDimension, TextureFormat, TextureUsages, TextureView};
use wgpu::util::DeviceExt;
use wgpu::VertexStepMode::Vertex;

