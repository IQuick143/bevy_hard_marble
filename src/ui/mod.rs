use bevy::prelude::*;

pub mod hud;

pub struct UIPlugin;

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, ui_setup)
			.add_plugins(hud::HUDPlugin)
		;
	}
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct UICamera;

fn ui_setup(
	mut commands: Commands
) {
	// Camera
	commands.spawn((
		Camera2dBundle {
			camera: Camera {
				order: -69, //fsr this is needed to be after the main camera (=> negative)
				..Default::default()
			},
			..Default::default()
		}, UICamera
	));
}
