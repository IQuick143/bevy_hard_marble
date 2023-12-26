use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier3d::prelude::RapierDebugRenderPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_plugins((EditorPlugin::default(), RapierDebugRenderPlugin::default()))
		;
	}
}
