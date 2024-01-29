//! This example illustrates the various features of Bevy UI.

use bevy::{
	a11y::{
		accesskit::{NodeBuilder, Role},
		AccessibilityNode,
	}, input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*
};

use bevy::app::{Plugin, App};

pub struct HUDPlugin;

#[derive(Resource, Default)]
/// Resource containing all data that the hud wants to display
struct HUDData {
	player_locked_momentum: bool,
	meme_data: Vec<String>,
}

#[derive(Component)]
struct MomentumLockIndicatorText;
#[derive(Component)]
struct TargetListText;

impl Plugin for HUDPlugin {
	fn build(&self, app: &mut App) {
		app
		.init_resource::<HUDData>()
		.add_systems(Startup, setup)
		.add_systems(Update, mouse_scroll)
		.add_systems(Update, (
			(
				update_hud_data_from_player_data,
				update_hud_data_from_memetics,
			), (
				update_hud_momentum_lock,
				update_target_list,
			),
		).chain())
		;
	}
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	// root node
	commands
		.spawn(NodeBundle {
			style: Style {
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				justify_content: JustifyContent::SpaceBetween,
				..default()
			},
			..default()
		})
		.with_children(|parent| {
			// left vertical fill (border)
			parent
				.spawn(NodeBundle {
					style: Style {
						width: Val::Percent(15.),
						border: UiRect::all(Val::Px(2.)),
						..default()
					},
					background_color: Color::rgb(0.65, 0.65, 0.65).into(),
					..default()
				})
				.with_children(|parent| {
					// left vertical fill (content)
					parent
						.spawn(NodeBundle {
							style: Style {
								width: Val::Percent(100.),
								display: Display::Flex,
								flex_direction: FlexDirection::Column,
								..default()
							},
							background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
							..default()
						})
						.with_children(|parent| {
							let indicator_text_style = TextStyle {
								font: asset_server.load("fonts/ComicMono.ttf"),
								font_size: 30.0,
								..default()
							};
							// Momentum Lock indicator
							parent.spawn((
								TextBundle::from_sections([
									TextSection::new("MOMENTUM LOCK: ", indicator_text_style.clone()),
									TextSection::from_style(indicator_text_style), // VALUE GOES HERE
								]).with_style(Style {
									margin: UiRect::all(Val::Px(5.)),
									..default()
								}),
								Label, // For accessibility
								MomentumLockIndicatorText,
							));
						});
				});
			// right vertical fill
			parent
				.spawn(NodeBundle {
					style: Style {
						flex_direction: FlexDirection::Column,
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						width: Val::Px(200.),
						..default()
					},
					background_color: Color::rgb(0.15, 0.15, 0.15).into(),
					..default()
				})
				.with_children(|parent| {
					// Title
					parent.spawn((
						TextBundle::from_section(
							"Amogus tasks or smth\nTODO: DO THIS THING +ěščřžýáíé",
							TextStyle {
								font: asset_server.load("fonts/ComicShannsV2.ttf"),
								font_size: 25.,
								..default()
							},
						),
						Label,
					));
					// List with hidden overflow
					parent
						.spawn(NodeBundle {
							style: Style {
								flex_direction: FlexDirection::Column,
								align_self: AlignSelf::Stretch,
								height: Val::Percent(50.),
								overflow: Overflow::clip_y(),
								..default()
							},
							background_color: Color::rgb(0.10, 0.10, 0.10).into(),
							..default()
						})
						.with_children(|parent| {
							// Moving panel
							parent
								.spawn((
									NodeBundle {
										style: Style {
											flex_direction: FlexDirection::Column,
											align_items: AlignItems::Center,
											..default()
										},
										..default()
									},
									ScrollingList::default(),
									AccessibilityNode(NodeBuilder::new(Role::List)),
								))
								.with_children(|parent| {
									let font = asset_server.load("fonts/ComicShannsV2.ttf");
									let style = TextStyle {
										font,
										font_size: 20.,
										..default()
									};
									parent.spawn((
										TextBundle::from_section(
											format!("THIS PLACEHOLDER IS COMPLETELY INTENTIONAL\n"), style
										),
										TargetListText,
										Label,
										AccessibilityNode(NodeBuilder::new(Role::ListItem)),
									));
								});
						});
				});
		});
}

#[derive(Component, Default)]
struct ScrollingList {
	position: f32,
}

fn mouse_scroll(
	mut mouse_wheel_events: EventReader<MouseWheel>,
	mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
	query_node: Query<&Node>,
) {
	for mouse_wheel_event in mouse_wheel_events.read() {
		for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
			let items_height = list_node.size().y;
			let container_height = query_node.get(parent.get()).unwrap().size().y;

			let max_scroll = (items_height - container_height).max(0.);

			let dy = match mouse_wheel_event.unit {
				MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
				MouseScrollUnit::Pixel => mouse_wheel_event.y,
			};

			scrolling_list.position += dy;
			scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
			style.top = Val::Px(scrolling_list.position);
		}
	}
}

fn update_hud_data_from_memetics(
	memetics: Query<&crate::meme::MemeContainer>,
	mut hud_data: ResMut<HUDData>
) {
	hud_data.meme_data.clear();

	for meme in memetics.iter() {
		hud_data.meme_data.push(format!("{:?} {:?}", meme.container_type, meme.contains_infohazard))
	}
}

fn update_hud_data_from_player_data(
	player: Query<&crate::player::PlayerMovement>,
	mut hud_data: ResMut<HUDData>
) {
	let Some(player) = ({
		let mut player_candidate = None;
		for a in player.iter() {
			player_candidate = Some(a);
			break;
		}
		player_candidate
	}) else {return;};

	hud_data.player_locked_momentum = player.locked_velocity;
}

fn update_hud_momentum_lock(
	mut indicator: Query<&mut Text, With<MomentumLockIndicatorText>>,
	hud_data: Res<HUDData>
) {
	for mut indicator in indicator.iter_mut() {
		// I trust my ass to not fuck this up in the hud creation
		indicator.sections[1].value = (if hud_data.player_locked_momentum {"ENABLED"} else {"DISABLED"}).into();
		indicator.sections[1].style.color = if hud_data.player_locked_momentum {Color::GREEN} else {Color::PINK};
	}
}

fn update_target_list(
	mut list_query: Query<&mut Text, With<TargetListText>>,
	hud_data: Res<HUDData>,
	asset_server: Res<AssetServer>
) {
	let font = asset_server.load("fonts/ComicShannsV2.ttf");
	let default_style = TextStyle {
		font,
		font_size: 20.,
		..default()
	};
	for mut list_text in list_query.iter_mut() {
		list_text.sections.clear();
		list_text.sections.extend(hud_data.meme_data.iter().map(
			|text| {
				TextSection {
					value: format!("{text}\n"), style: default_style.clone()
				}
			}
		));
	}
}
