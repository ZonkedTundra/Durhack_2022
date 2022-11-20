use rapier2d::{na::ArrayStorage, prelude::*};
use rsrl::{
    domains::{Domain, Observation},
    spaces::{discrete::Ordinal, real::Interval, ProductSpace},
};
use std::{f32::consts::PI, f64};

const MOTOR_FACTOR: f32 = 0.5;
const MOTOR_MAX: f64 = 2. * f64::consts::PI;

const REWARD_STEP: f64 = -1.0;
const REWARD_GOAL: f64 = 0.0;

// pub struct Simulation {
//     body_handle: RigidBodyHandle,
//     leg_upper_left_handle: RigidBodyHandle,
//     leg_upper_right_handle: RigidBodyHandle,
//     leg_lower_left_handle: RigidBodyHandle,
//     leg_lower_right_handle: RigidBodyHandle,
//     joint_upper_left_handle: ImpulseJointHandle,
//     joint_upper_right_handle: ImpulseJointHandle,
//     joint_lower_left_handle: ImpulseJointHandle,
//     joint_lower_right_handle: ImpulseJointHandle,
//     rigid_body_set: RigidBodySet,
//     collider_set: ColliderSet,
//     impulse_joint_set: ImpulseJointSet,
//     gravity: nalgebra::Matrix<f32, nalgebra::Const<2>, nalgebra::Const<1>, ArrayStorage<f32, 2, 1>>,
//     integration_parameters: IntegrationParameters,
//     physics_pipeline: PhysicsPipeline,
//     island_manager: IslandManager,
//     broad_phase: BroadPhase,
//     narrow_phase: NarrowPhase,
//     multibody_joint_set: MultibodyJointSet,
//     ccd_solver: CCDSolver,
//     physics_hooks: (),
//     event_handler: (),
// }

const ACTIONS: [(f32, f32, f32, f32); 81] = [
    (-1., -1., -1., -1.),
    (-1., -1., -1., 0.),
    (-1., -1., -1., 1.),
    (-1., -1., 0., -1.),
    (-1., -1., 0., 0.),
    (-1., -1., 0., 1.),
    (-1., -1., 1., -1.),
    (-1., -1., 1., 0.),
    (-1., -1., 1., 1.),
    (-1., 0., -1., -1.),
    (-1., 0., -1., 0.),
    (-1., 0., -1., 1.),
    (-1., 0., 0., -1.),
    (-1., 0., 0., 0.),
    (-1., 0., 0., 1.),
    (-1., 0., 1., -1.),
    (-1., 0., 1., 0.),
    (-1., 0., 1., 1.),
    (-1., 1., -1., -1.),
    (-1., 1., -1., 0.),
    (-1., 1., -1., 1.),
    (-1., 1., 0., -1.),
    (-1., 1., 0., 0.),
    (-1., 1., 0., 1.),
    (-1., 1., 1., -1.),
    (-1., 1., 1., 0.),
    (-1., 1., 1., 1.),
    (0., -1., -1., -1.),
    (0., -1., -1., 0.),
    (0., -1., -1., 1.),
    (0., -1., 0., -1.),
    (0., -1., 0., 0.),
    (0., -1., 0., 1.),
    (0., -1., 1., -1.),
    (0., -1., 1., 0.),
    (0., -1., 1., 1.),
    (0., 0., -1., -1.),
    (0., 0., -1., 0.),
    (0., 0., -1., 1.),
    (0., 0., 0., -1.),
    (0., 0., 0., 0.),
    (0., 0., 0., 1.),
    (0., 0., 1., -1.),
    (0., 0., 1., 0.),
    (0., 0., 1., 1.),
    (0., 1., -1., -1.),
    (0., 1., -1., 0.),
    (0., 1., -1., 1.),
    (0., 1., 0., -1.),
    (0., 1., 0., 0.),
    (0., 1., 0., 1.),
    (0., 1., 1., -1.),
    (0., 1., 1., 0.),
    (0., 1., 1., 1.),
    (1., -1., -1., -1.),
    (1., -1., -1., 0.),
    (1., -1., -1., 1.),
    (1., -1., 0., -1.),
    (1., -1., 0., 0.),
    (1., -1., 0., 1.),
    (1., -1., 1., -1.),
    (1., -1., 1., 0.),
    (1., -1., 1., 1.),
    (1., 0., -1., -1.),
    (1., 0., -1., 0.),
    (1., 0., -1., 1.),
    (1., 0., 0., -1.),
    (1., 0., 0., 0.),
    (1., 0., 0., 1.),
    (1., 0., 1., -1.),
    (1., 0., 1., 0.),
    (1., 0., 1., 1.),
    (1., 1., -1., -1.),
    (1., 1., -1., 0.),
    (1., 1., -1., 1.),
    (1., 1., 0., -1.),
    (1., 1., 0., 0.),
    (1., 1., 0., 1.),
    (1., 1., 1., -1.),
    (1., 1., 1., 0.),
    (1., 1., 1., 1.),
];

pub struct Simulation {
    body_handle: RigidBodyHandle,
    leg_upper_left_handle: RigidBodyHandle,
    leg_upper_right_handle: RigidBodyHandle,
    leg_lower_left_handle: RigidBodyHandle,
    leg_lower_right_handle: RigidBodyHandle,
    joint_upper_left_handle: ImpulseJointHandle,
    joint_upper_right_handle: ImpulseJointHandle,
    joint_lower_left_handle: ImpulseJointHandle,
    joint_lower_right_handle: ImpulseJointHandle,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    gravity: nalgebra::Matrix<f32, nalgebra::Const<2>, nalgebra::Const<1>, ArrayStorage<f32, 2, 1>>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
}

impl Default for Simulation {
    fn default() -> Self {
        /* code adapted from https://rapier.rs/docs/user_guides/rust/getting_started/#basic-simulation-example */
        /* Create the sets */
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut impulse_joint_set = ImpulseJointSet::new();

        /* Create the ground. */
        let ground_collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        collider_set.insert(ground_collider);

        /* Create the horse */
        // Body
        let body = RigidBodyBuilder::dynamic()
            .translation(vector![0., 3.])
            .build();
        let body_collider = ColliderBuilder::cuboid(2., 4.)
            .collision_groups(InteractionGroups::new(0b000001.into(), 0b11110.into()));
        let body_handle = rigid_body_set.insert(body);
        collider_set.insert_with_parent(body_collider, body_handle, &mut rigid_body_set);

        // Legs
        let leg_upper_left = RigidBodyBuilder::dynamic()
            .translation(vector![-1., 3.])
            .build();
        let leg_upper_left_collider = ColliderBuilder::cuboid(1., 3.)
            .collision_groups(InteractionGroups::new(0b00010.into(), 0b00001.into()));
        let leg_upper_left_handle = rigid_body_set.insert(leg_upper_left);
        collider_set.insert_with_parent(
            leg_upper_left_collider,
            leg_upper_left_handle,
            &mut rigid_body_set,
        );

        let leg_upper_right = RigidBodyBuilder::dynamic()
            .translation(vector![2., 3.])
            .build();
        let leg_upper_right_collider = ColliderBuilder::cuboid(1., 3.)
            .collision_groups(InteractionGroups::new(0b00100.into(), 0b00001.into()));
        let leg_upper_right_handle = rigid_body_set.insert(leg_upper_right);
        collider_set.insert_with_parent(
            leg_upper_right_collider,
            leg_upper_right_handle,
            &mut rigid_body_set,
        );

        let leg_lower_left = RigidBodyBuilder::dynamic()
            .translation(vector![-1., 0.])
            .build();
        let leg_lower_left_collider = ColliderBuilder::cuboid(1., 3.)
            .collision_groups(InteractionGroups::new(0b01000.into(), 0b00001.into()));
        let leg_lower_left_handle = rigid_body_set.insert(leg_lower_left);
        collider_set.insert_with_parent(
            leg_lower_left_collider,
            leg_lower_left_handle,
            &mut rigid_body_set,
        );

        let leg_lower_right = RigidBodyBuilder::dynamic()
            .translation(vector![2., 0.])
            .build();
        let leg_lower_right_collider = ColliderBuilder::cuboid(1., 3.)
            .collision_groups(InteractionGroups::new(0b10000.into(), 0b00001.into()));
        let leg_lower_right_handle = rigid_body_set.insert(leg_lower_right);
        collider_set.insert_with_parent(
            leg_lower_right_collider,
            leg_lower_right_handle,
            &mut rigid_body_set,
        );

        // Leg joints
        let joint_upper_left = RevoluteJointBuilder::new()
            .local_anchor1(point![0., 0.])
            .local_anchor2(point![1., 3.]);
        let joint_upper_left_handle =
            impulse_joint_set.insert(body_handle, leg_upper_left_handle, joint_upper_left, true);

        let joint_upper_right = RevoluteJointBuilder::new()
            .local_anchor1(point![2., 0.])
            .local_anchor2(point![0., 3.]);
        let joint_upper_right_handle =
            impulse_joint_set.insert(body_handle, leg_upper_right_handle, joint_upper_right, true);

        let joint_lower_left = RevoluteJointBuilder::new()
            .local_anchor1(point![1., 0.])
            .local_anchor2(point![1., 3.]);
        let joint_lower_left_handle =
            impulse_joint_set.insert(body_handle, leg_lower_left_handle, joint_lower_left, true);

        let joint_lower_right = RevoluteJointBuilder::new()
            .local_anchor1(point![0., 0.])
            .local_anchor2(point![0., 3.]);
        let joint_lower_right_handle =
            impulse_joint_set.insert(body_handle, leg_lower_right_handle, joint_lower_right, true);

        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, -9.81];
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        /* Run the game loop, stepping the simulation once per frame. */
        // for _ in 0..200 {
        //     physics_pipeline.step(
        //         &gravity,
        //         &integration_parameters,
        //         &mut island_manager,
        //         &mut broad_phase,
        //         &mut narrow_phase,
        //         &mut rigid_body_set,
        //         &mut collider_set,
        //         &mut impulse_joint_set,
        //         &mut multibody_joint_set,
        //         &mut ccd_solver,
        //         &physics_hooks,
        //         &event_handler,
        //     );

        //     let ball_body = &rigid_body_set[ball_body_handle];
        //     println!("Ball altitude: {}", ball_body.translation().y);
        // }

        Self {
            body_handle,
            leg_upper_left_handle,
            leg_upper_right_handle,
            leg_lower_left_handle,
            leg_lower_right_handle,
            joint_upper_left_handle,
            joint_upper_right_handle,
            joint_lower_left_handle,
            joint_lower_right_handle,
            rigid_body_set,
            collider_set,
            impulse_joint_set,
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            multibody_joint_set,
            ccd_solver,
            physics_hooks,
            event_handler,
        }
    }
}

impl Simulation {
    fn new() -> Self {
        Self::default()
    }
}

impl Domain for Simulation {
    type StateSpace = ProductSpace<Interval>;
    type ActionSpace = Ordinal;

    fn emit(&self) -> Observation<Vec<f64>> {
        let body = self.rigid_body_set.get(self.body_handle).unwrap();
        let height = body.translation().y;
        let body_rotation = body.rotation().angle();
        let leg_upper_left_rotation = self
            .rigid_body_set
            .get(self.leg_upper_left_handle)
            .unwrap()
            .rotation()
            .angle();
        let leg_upper_right_rotation = self
            .rigid_body_set
            .get(self.leg_upper_right_handle)
            .unwrap()
            .rotation()
            .angle();
        let leg_lower_left_rotation = self
            .rigid_body_set
            .get(self.leg_lower_left_handle)
            .unwrap()
            .rotation()
            .angle();
        let leg_lower_right_rotation = self
            .rigid_body_set
            .get(self.leg_lower_right_handle)
            .unwrap()
            .rotation()
            .angle();

        let upper_left_angle = (body_rotation - leg_upper_left_rotation + PI) % (2. * PI) - PI;
        let upper_right_angle = (body_rotation - leg_upper_right_rotation + PI) % (2. * PI) - PI;
        let lower_left_angle =
            (leg_lower_left_rotation - leg_upper_left_rotation + PI) % (2. * PI) - PI;
        let lower_right_angle =
            (leg_lower_left_rotation - leg_upper_right_rotation + PI) % (2. * PI) - PI;

        let state = vec![
            height,
            body_rotation,
            upper_left_angle,
            upper_right_angle,
            lower_left_angle,
            lower_right_angle,
        ]
        .iter()
        .map(|&n| n as f64)
        .collect();

        if body.translation().x > 100. {
            Observation::Terminal(state)
        } else {
            Observation::Full(state)
        }
    }

    fn step(&mut self, &a: &usize) -> (Observation<Vec<f64>>, rsrl::domains::Reward) {
        let action = ACTIONS[a];

        self.impulse_joint_set
            .iter_mut()
            .for_each(|(handle, joint)| {
                let joint_upper_left_id = self.joint_upper_left_handle.0;
                let joint_upper_right_id = self.joint_upper_right_handle.0;
                let joint_lower_left_id = self.joint_lower_left_handle.0;
                let joint_lower_right_id = self.joint_lower_right_handle.0;

                match handle.0 {
                    a if a == joint_upper_left_id => {
                        joint
                            .data
                            .set_motor_velocity(JointAxis::X, action.0, MOTOR_FACTOR);
                    }
                    a if a == joint_upper_right_id => {
                        joint
                            .data
                            .set_motor_velocity(JointAxis::X, action.1, MOTOR_FACTOR);
                    }
                    a if a == joint_lower_left_id => {
                        joint
                            .data
                            .set_motor_velocity(JointAxis::X, action.2, MOTOR_FACTOR);
                    }
                    a if a == joint_lower_right_id => {
                        joint
                            .data
                            .set_motor_velocity(JointAxis::X, action.3, MOTOR_FACTOR);
                    }
                    _ => (),
                };
            });

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
            &self.physics_hooks,
            &self.event_handler,
        );

        let to = self.emit();

        let reward = if to.is_terminal() {
            REWARD_GOAL
        } else {
            REWARD_STEP
        };

        (to, reward)
    }

    fn action_space(&self) -> Ordinal {
        Ordinal::new(81)
    }

    fn state_space(&self) -> Self::StateSpace {
        ProductSpace::empty()
            + Interval::bounded(-MOTOR_MAX, MOTOR_MAX)
            + Interval::bounded(-MOTOR_MAX, MOTOR_MAX)
            + Interval::bounded(-MOTOR_MAX, MOTOR_MAX)
            + Interval::bounded(-MOTOR_MAX, MOTOR_MAX)
    }
}
