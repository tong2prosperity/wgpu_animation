use rapier2d::prelude::*;

pub struct Physics {

}

impl Physics {
    pub fn new() -> Self {
        
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
    
        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
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
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = DefaultBroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let mut query_pipeline = QueryPipeline::new();
        Self{}
    }
}