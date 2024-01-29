mod player;
mod state;
#[cfg(debug_assertions)]
mod debug;
mod input;
mod meme;
mod ui;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
	let mut app: App = App::new();
	app.add_state::<state::GameState>();
	app.add_plugins(DefaultPlugins);
	app.insert_resource(RapierConfiguration {
		gravity: Vec3::new(0.0, -9.81, 0.0),
		..Default::default()
	});
	app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
	#[cfg(debug_assertions)]
	{app.add_plugins(debug::DebugPlugin);}
	app.add_systems(Startup, (test_setup, setup_test_room));
	app.add_plugins((
		ui::UIPlugin,
		input::InputPlugin,
		player::PlayerPlugin,
	));
	app.run();
}

fn test_setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
) {
	let cube = meshes.add(bevy::render::mesh::shape::Cube::default().into());
	//commands.spawn(Camera3dBundle {
	//	transform: Transform::from_translation(Vec3::new(-5.0, 2.0, 1.0)).looking_at(Vec3::ZERO, Vec3::Y),
	//	..Default::default()
	//});
	commands.spawn((
		PbrBundle {
			mesh: cube,
			..Default::default()
		},
		(RigidBody::Dynamic, Velocity::zero(), GravityScale(1.0), Sleeping::disabled(),
		Collider::cuboid(0.5, 0.5, 0.5), ColliderMassProperties::Mass(1.0))
	));
}

fn setup_test_room(
	mut commands: Commands,
	assets: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut game_state: ResMut<NextState<state::GameState>>,
) {
	let material = materials.add(StandardMaterial {
		base_color_texture: Some(assets.load::<Image>("grid.png")),
		alpha_mode: AlphaMode::Blend,
		unlit: true,
		..default()
	});
	let plane = meshes.add(bevy::render::mesh::shape::Plane::from_size(1000.0).into());
	commands.spawn((
		PbrBundle {
			mesh: plane,
			material,
			..Default::default()
		},
		(RigidBody::Fixed, Sleeping::default(), Collider::halfspace(Vec3::Y).unwrap()),
		meme::MemeContainer {
			container_type: meme::ContainerType::Electronic,
			container_activity: meme::ContainerActivity::Passive,
			contains_infohazard: true
		},
	));
	game_state.0 = Some(state::GameState::InLevel);
}
