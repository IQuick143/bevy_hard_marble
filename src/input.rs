use bevy::prelude::*;

pub struct InputPlugin;

#[derive(Resource, Debug, PartialEq)]
/// A mapping of all the actions to KeyCodes, use this for KeyCode constants + storing keymap settings
pub struct InputMap {
	pub velocity_lock: KeyCode,
}

impl Default for InputMap {
	fn default() -> Self {
		Self {
			velocity_lock: KeyCode::ShiftLeft
		}
	}
}

#[derive(Resource, Debug, PartialEq)]
/// A settings structure for all your magic input constants
pub struct InputSettings {
	// TODO: Move all configurable input values (ex. sensitivity) to here
	/// Camera sensitivity/turning speed [radian/mouse_unit]
	pub camera_sensitivity: f32
}

impl Default for InputSettings {
	fn default() -> Self {
		Self {
			camera_sensitivity: 0.005
		}
	}
}

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app
		.init_resource::<InputMap>()
		.init_resource::<InputSettings>()
		;
	}
}
