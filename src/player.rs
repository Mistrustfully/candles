use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::shaders::light::Lights;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
struct Player;

fn spawn_player(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let texture_handle = asset_server.load("player.png");
	let texture_atlas =
		TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 3, 4, None, None);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);
	commands.spawn((
		SpriteSheetBundle {
			texture_atlas: texture_atlas_handle,
			transform: Transform::from_xyz(0.0, 0.0, 10.0),
			..default()
		},
		RigidBody::Dynamic,
		LockedAxes::ROTATION_LOCKED,
		Velocity::zero(),
		Collider::convex_hull(&[
			Vec2::new(-8.0, -8.0),
			Vec2::new(-8.0, -2.0),
			Vec2::new(8.0, -8.0),
			Vec2::new(8.0, -2.0),
		])
		.unwrap(),
		Player,
	));
}

fn camera_follows_player(
	mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
	player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
	let mut camera_transform = camera_query.get_single_mut().expect("No player camera?");
	let player_transform = player_query.get_single().expect("No player?");

	camera_transform.translation = Vec3::new(
		player_transform.translation.x,
		player_transform.translation.y,
		camera_transform.translation.z,
	);
}

fn light_follows_player(player_query: Query<&Transform, With<Player>>, mut lights: ResMut<Lights>) {
	let player_transform = player_query.get_single().expect("No player?").translation;
	lights.0[0] = Vec4::new(
		player_transform.x,
		player_transform.y,
		player_transform.z,
		0.5,
	);
}

fn player_movement(
	keyboard_input: Res<Input<KeyCode>>,
	mut velocity_query: Query<&mut Velocity, With<Player>>,
) {
	for mut rb_vels in velocity_query.iter_mut() {
		let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
		let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
		let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
		let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

		let x_axis = -(left as i8) + right as i8;
		let y_axis = -(down as i8) + up as i8;

		let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
		if move_delta != Vec2::ZERO {
			move_delta /= move_delta.length();
		}

		// Update the velocity on the rigid_body_component,
		// the bevy_rapier plugin will update the Sprite transform.
		rb_vels.linvel = move_delta * 128.0;
	}
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(spawn_player)
			.add_system(player_movement)
			.add_system(light_follows_player)
			.add_system_to_stage(CoreStage::PostUpdate, camera_follows_player);
		// We add the
		// camera_follows_player system to PhysicStages::Writeback so the physic calculations are
		// done before we move the camera.
	}
}
