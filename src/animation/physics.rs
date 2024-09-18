use rapier2d::prelude::*;
use log::info;
use nalgebra::Vector2;

pub struct AnimateHandle {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

pub struct Physics {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub gravity: nalgebra::Vector2<f32>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub animate_handle: AnimateHandle,
}

impl Physics {
    pub fn new() -> Self {
        
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let mut animate_handle = AnimateHandle {
            rigid_body_handle: RigidBodyHandle::invalid(),
            collider_handle: ColliderHandle::invalid(),
        };
    
        /* Create the ground. */
        let collider = ColliderBuilder::ball(0.5).build();
        collider_set.insert(collider);
        
        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(nalgebra::vector![0.0, 10.0])
            .build();
        
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        let ball_collider_handle = collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);
        animate_handle.rigid_body_handle = ball_body_handle;
        animate_handle.collider_handle = ball_collider_handle;
    
        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, -9.81];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        Self{
            rigid_body_set,
            collider_set,
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            animate_handle,
        }
    }

    pub fn step(&mut self) -> glam::Vec2 {
        let physics_hooks = ();
        let event_handler = ();
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &physics_hooks,
            &event_handler,
        );
        
        let ball = self.collider_set.get(self.animate_handle.collider_handle).unwrap();
        let ball_rb = self.rigid_body_set.get(self.animate_handle.rigid_body_handle).unwrap();
        let p = ball.translation();
        let p_rb = ball_rb.translation();
        info!("p: {:?}, p_rb: {:?}", p, p_rb);
        let p_glam = glam::Vec2::new(p.x, p.y);
        return p_glam;
    }
}