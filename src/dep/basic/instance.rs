use std::mem;
use rand::Rng;
use wgpu::util::DeviceExt;

pub struct Instance {
    pub position: glam::Vec3,
    pub rotation: glam::Quat,
    pub transform: glam::Mat4,
    pub theta: f32,
    pub speed: f32,
    pub scale: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32;4];4],
    pub origin3d: [f32;3],
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance3DRaw {
    pub model: [[f32;3];3],
}


pub struct InstanceManager {
    pub instances: Vec<Instance>,
    pub instance_buffer: Option<wgpu::Buffer>,
}

impl InstanceManager {
    pub fn new() -> Self {
        InstanceManager {
            instances: vec![],
            instance_buffer: None,
        }
    }

    pub fn make_up_instances(&mut self) {
        let mut rng = rand::thread_rng();
        let points = vec![
            glam::Vec3::new(0.0, 1.0, 0.0),
            // glam::Vec3::new(-0.6, 0.6, 0.0),
            // glam::Vec3::new(-0.6, -0.6, 0.0),
            // glam::Vec3::new(0.6, -0.6, 0.0),
            // glam::Vec3::new(0.6, 0.6, 0.0),
        ];
        for i in 0..points.len() {
            let position = points[i];
            let rotation = glam::Quat::from_rotation_z(rng.gen_range(0.0..std::f32::consts::PI));
            self.add_instance(Instance::new(position, rotation));
        }
    }

    pub fn add_instance(&mut self, instance: Instance) {
        self.instances.push(instance);
    }

    pub fn init_buffer(&mut self, device: &wgpu::Device) {
        let instances_data = self.instances.iter().map(|instance| instance.to_raw()).collect::<Vec<InstanceRaw>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances_data),
            usage: wgpu::BufferUsages::VERTEX,

        });
        self.instance_buffer = Some(instance_buffer);
    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        self.instance_buffer.as_ref().unwrap()
    }
}

impl Instance {
    pub fn new(position: glam::Vec3, rotation: glam::Quat) -> Self {
        let transform = glam::Mat4::from_translation(position) * glam::Mat4::from_quat(rotation);
        Instance {
            position,
            rotation,
            transform,
            theta: 0.0,
            speed: 0.0,
            scale: 0.0,
        }
    }

    pub fn to_raw(&self) -> InstanceRaw {
        let coll = (glam::Mat4::from_translation(self.position) * glam::Mat4::from_quat(self.rotation)).to_cols_array_2d();
        InstanceRaw {
            model: coll,
            origin3d: self.position.to_array(),
        }
    }
}

impl InstanceRaw{
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32;4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32;8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32;12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: mem::size_of::<[f32;16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                }
            ],
        }
    }
}

impl Instance3DRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance3DRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32;6]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
