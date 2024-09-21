pub mod renderer;
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


use wgpu::{Device, SurfaceConfiguration, TextureView};

