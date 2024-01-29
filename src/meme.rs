use bevy::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ContainerType {
	Biological, Electronic
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ContainerActivity {
	Active, Passive
}

/// Component denoting an entity is able to contain memetics
#[derive(Component)]
pub struct MemeContainer {
	pub container_type: ContainerType,
	pub container_activity: ContainerActivity,
	pub contains_infohazard: bool,
}
