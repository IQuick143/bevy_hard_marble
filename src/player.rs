use std::f32::consts::PI;

use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_rapier3d::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Player;

/// Contains all game-logic information (input and kinematics) about player movement
/// More relevant data can be found in an attached `Transform` and `KinematicCharacterController`
#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub struct PlayerMovement {
	/// How long we've been accelerating, determines max speed among other things
	pub time_accelerating: f32,
	/// Velocity vector the player is trying to move at
	pub desired_velocity: Vec3,
	/// Whether the physics engine said we are touching ground
	pub grounded: bool,
	/// Whether the velocity vector should be locked from being changed by inputs
	pub locked_velocity: bool,
}

#[derive(Resource, Clone, Copy, PartialEq, Debug, Default)]
pub struct MouseMovement {
	/// An averaged speed value, roughly corresponding to how quickly mouse is moving around
	pub average_speed: f32,
	/// An averaged velocity vector, roughly corresponding to overall mouse movement
	pub average_velocity: Vec2,
}

#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub struct PlayerCamera {
	/// Angle between -pi/2 and +pi/2 measuring the pitch of the camera
	pitch: f32
}

#[derive(Resource, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PlayerMesh(Handle<Mesh>);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<PlayerMesh>()
			.init_resource::<MouseMovement>()
			.add_systems(Startup, load_player_mesh)
			.add_systems(OnEnter(crate::state::GameState::InLevel), spawn_player)
			.add_systems(Update, (
				process_mouse_movement,
				(rotate_player, player_move_input),
				player_kinematics.before(PhysicsSet::SyncBackend)
			).chain())
			.add_systems(Update, read_result_system.after(PhysicsSet::Writeback))
		;
	}
}

fn load_player_mesh(
	mut meshes: ResMut<Assets<Mesh>>,
	mut player_mesh: ResMut<PlayerMesh>,
) {
	player_mesh.0 = meshes.add(bevy::render::mesh::shape::Capsule {
		radius: 0.5,
		rings: 2,
		depth: 2.,
		latitudes: 16,
		longitudes: 5,
		uv_profile: shape::CapsuleUvProfile::Aspect,
	}.into());
}

fn spawn_player(
	mut commands: Commands,
	player_mesh: Res<PlayerMesh>,
) {
	commands.spawn((
		PbrBundle {
			mesh: player_mesh.0.clone(),
			..Default::default()
		},
		(KinematicCharacterController {
			offset: CharacterLength::Absolute(0.1),
			slide: true,
			apply_impulse_to_dynamic_bodies: true,
			..Default::default()
		},
		Friction {
			coefficient: 0.1,
			combine_rule: CoefficientCombineRule::Min,
		},
		Collider::capsule_y(1.0, 0.5), ColliderMassProperties::Mass(20.0),
		Player, PlayerMovement::default())
	)).with_children(|builder| {
		builder.spawn((
			PlayerCamera::default(),
			Camera3dBundle {
				transform: Transform::from_translation(Vec3::Y),
				..Default::default()
			}
		));
	});
}

fn process_mouse_movement(
	mut mouse_inputs: EventReader<MouseMotion>,
	mut mouse_statistics: ResMut<MouseMovement>,
) {
	let mut dmouse: Vec2 = Vec2::ZERO;

	for input in mouse_inputs.read() {
		dmouse += input.delta;
	}

	// Exponentially tracked moving average
	let percentage = 0.05;
	mouse_statistics.average_speed *= 1.0 - percentage;
	mouse_statistics.average_velocity *= 1.0 - percentage;
	mouse_statistics.average_speed += dmouse.length() * percentage;
	mouse_statistics.average_velocity += dmouse * percentage;
}

fn rotate_player(
	mut player: Query<&mut Transform, With<Player>>,
	mut camera: Query<(&mut Transform, &mut PlayerCamera), Without<Player>>,
	mut mouse_inputs: EventReader<MouseMotion>,
) {
	let mut dmouse: Vec2 = Vec2::ZERO;

	for input in mouse_inputs.read() {
		dmouse += input.delta;
	}

	// Invert y
	dmouse.y *= -1.0;
	// TODO: Figure out sensitivity
	let delta_angle = dmouse * 0.005;

	if delta_angle.length_squared() > 0.00001 {
		for mut transform in player.iter_mut() {
			// Rotate player
			transform.rotate_local_y(-delta_angle.x);
		}
		for (mut transform, mut camera) in camera.iter_mut() {
			// Rotate camera
			camera.pitch = f32::clamp(camera.pitch + delta_angle.y, -PI/2.0, PI/2.0) ;
			transform.rotation = Quat::from_rotation_x(camera.pitch);
		}
	}
}

fn player_move_input(
	mut player: Query<(&Transform, &mut PlayerMovement), With<Player>>,
	camera: Query<&PlayerCamera, Without<Player>>,
	mouse_data: Res<MouseMovement>,
	time: Res<Time>,
) {
	let Ok(camera_angle) = camera.get_single() else {return;};
	let dt = time.delta_seconds();

	let max_acceleration = 15.0;
	let min_acceleration = 3.0;
	let acceleration = {
		let t = 0.5 * camera_angle.pitch / PI;
		max_acceleration * t + min_acceleration * (1.0 - t)
	};

	let min_speed = dt * 300.0;

	// Gotta check we are circling with cursor
	if mouse_data.average_speed > mouse_data.average_velocity.length() && mouse_data.average_speed > min_speed {
		for (transform, mut player) in player.iter_mut() {
			let min_time_acceleration = 0.5;
			if player.time_accelerating < min_time_acceleration {
				player.time_accelerating = min_time_acceleration;
				player.desired_velocity += min_time_acceleration * acceleration * transform.forward()
			}
			player.time_accelerating += dt;
			player.desired_velocity += transform.forward() * dt * acceleration;
			// Rotate velocity vector towards transform.forward
			let axis = player.desired_velocity.cross(transform.forward()).y * transform.up();
			let delta = dt * player.desired_velocity.cross(axis);
			// Ultra yee-yee ass rotation, might work tho
			let speed = player.desired_velocity.length();
			if speed > 0.1 {
				player.desired_velocity -= delta / speed;
			}

			player.desired_velocity = player.desired_velocity.clamp_length_max(player.time_accelerating * max_acceleration)
		}
	} else {
		for (_, mut player) in player.iter_mut() {
			player.time_accelerating *= 0.9;
			player.desired_velocity *= 0.95;
		}
	}
}

fn player_kinematics(
	mut player: Query<(&mut PlayerMovement, &mut KinematicCharacterController), With<Player>>,
	time: Res<Time>,
) {
	let dt = time.delta_seconds();
	for (mut player, mut kinematic_body) in player.iter_mut() {
		//println!("{:?}", player.desired_velocity);
		if player.grounded {
			if player.desired_velocity.y <= 0.0 {
				player.desired_velocity.y = 0.01;
			}
		} else {
			player.desired_velocity.y -= dt * 9.81;
		}
		kinematic_body.translation = Some(player.desired_velocity * dt);
	}
}

fn read_result_system(mut controllers: Query<(&mut PlayerMovement, &KinematicCharacterControllerOutput)>) {
	for (mut player, output) in controllers.iter_mut() {
		player.grounded = output.grounded;
	}
}
