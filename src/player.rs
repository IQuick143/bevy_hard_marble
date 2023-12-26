use std::f32::consts::PI;

use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_rapier3d::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Player;
#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub struct PlayerCamera {
	pitch: f32
}

#[derive(Resource, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PlayerMesh(Handle<Mesh>);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<PlayerMesh>()
			.add_systems(Startup, load_player_mesh)
			.add_systems(OnEnter(crate::state::GameState::InLevel), spawn_player)
			.add_systems(Update, rotate_player)
			.add_systems(Update, move_player)
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
			..Default::default()
		}, Collider::capsule_y(1.0, 0.5),
		ColliderMassProperties::Mass(20.0), Player)
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

fn rotate_player(
	mut player: Query<&mut Transform, With<Player>>,
	mut camera: Query<(&mut Transform, &mut PlayerCamera), Without<Player>>,
	mut mouse_inputs: EventReader<MouseMotion>,
) {
	let mut dmouse: Vec2 = Vec2::ZERO;

	for input in mouse_inputs.read() {
		dmouse += input.delta;
	}

	// TODO: Figure out sensitivity
	let delta_angle = dmouse * 0.001;

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

fn move_player(
//	mut player: Query<&mut Transform, With<Player>>,
//	mut keyboard_inputs: EventReader<MouseMotion>,
) {

}
