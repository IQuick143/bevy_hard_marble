use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
	let mut app: App = App::new();
	app.add_plugins(DefaultPlugins);
	app.insert_resource(RapierConfiguration {
		gravity: Vec3::new(0.0, -9.81, 0.0),
		..Default::default()
	});
	app.add_plugins((
		RapierPhysicsPlugin::<NoUserData>::default(),
		RapierDebugRenderPlugin::default()
	));
	app.add_systems(Startup, test_setup);
	app.run();
}

fn test_setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
) {
	let cube = meshes.add(bevy::render::mesh::shape::Cube::default().into());
	commands.spawn(Camera3dBundle {
		transform: Transform::from_translation(Vec3::new(-5.0, 0.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
		..Default::default()
	});
	commands.spawn((
		PbrBundle {
			mesh: cube,
			..Default::default()
		},
		(RigidBody::Dynamic, Velocity::zero(), GravityScale(1.0), Sleeping::disabled(),
		Collider::cuboid(0.5, 0.5, 0.5), ColliderMassProperties::Mass(1.0))
	));
}
