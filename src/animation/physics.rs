use rapier2d::prelude::*;

pub struct Physics {

}

impl Physics {
    pub fn new() -> Self {
        
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
    
        /* Create the ground. */
        let collider = ColliderBuilder::ball(0.5).build();
        collider_set.insert(collider);
    
        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0])
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);
    
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
        Self{}
    }
}