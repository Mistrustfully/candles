mod collectables;
mod collision;
mod player;
mod shaders;
mod tilemap;

use bevy::{
	prelude::*,
	render::{
		camera::ScalingMode,
		render_resource::{
			Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
		},
	},
	sprite::Mesh2dHandle,
	window::WindowResized,
};
use collectables::*;
use collision::*;
use player::*;
use tilemap::*;

const V_RESOLUTION: f32 = 144.0;
const H_RESOLUTION: f32 = 144.0;

const V_WIN_SIZE: f32 = V_RESOLUTION * 6.0;
const H_WIN_SIZE: f32 = H_RESOLUTION * 6.0;

pub const PALETTE: [Color; 4] = [
	Color::hsl(0.0, 0.0, 0.12),
	Color::hsl(0.0, 0.0, 0.37),
	Color::hsl(0.0, 0.0, 0.62),
	Color::hsl(0.0, 0.0, 0.87),
];

#[derive(Resource)]
pub struct MainRenderTarget(Handle<Image>);

#[derive(Component)]
pub struct ShouldResize;

pub fn create_window_quad(window: &Window) -> Mesh {
	Mesh::from(shape::Quad::new(Vec2::new(
		window.physical_width() as f32,
		window.physical_height() as f32,
	)))
}

fn create_cameras(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
	let size = Extent3d {
		width: H_WIN_SIZE as u32,
		height: V_WIN_SIZE as u32,
		..default()
	};

	// This is the texture that will be rendered to.
	let mut image = Image {
		texture_descriptor: TextureDescriptor {
			label: None,
			size,
			dimension: TextureDimension::D2,
			format: TextureFormat::Bgra8UnormSrgb,
			mip_level_count: 1,
			sample_count: 1,
			usage: TextureUsages::TEXTURE_BINDING
				| TextureUsages::COPY_DST
				| TextureUsages::RENDER_ATTACHMENT,
		},
		..default()
	};

	// fill image.data with zeroes
	image.resize(size);
	let image_handle = images.add(image);

	let camera = Camera2dBundle {
		projection: OrthographicProjection {
			scaling_mode: ScalingMode::FixedVertical(2.0),
			scale: V_RESOLUTION / 2.0,
			..default()
		},
		camera: Camera {
			target: bevy::render::camera::RenderTarget::Image(image_handle.clone_weak()),
			..default()
		},
		..default()
	};

	commands.insert_resource(MainRenderTarget(image_handle));
	commands.spawn(camera).insert(PlayerCamera);
}

fn update(
	windows: Res<Windows>,
	mut resize_reader: EventReader<WindowResized>,
	mut mesh_assets: ResMut<Assets<Mesh>>,
	needs_new_quad: Query<&Mesh2dHandle, With<ShouldResize>>,
) {
	let main_window_id = windows.get_primary().expect("Should have window").id();

	for event in resize_reader.iter() {
		if event.id != main_window_id {
			continue;
		}

		let window = windows.get(event.id).expect("Main window should exist");

		for resize_me in needs_new_quad.iter() {
			let mesh = create_window_quad(&window);
			*mesh_assets.get_mut(&resize_me.0).expect("Should find mesh") = mesh;
		}
	}
}

fn main() {
	App::new()
		.insert_resource(ClearColor(PALETTE[0]))
		.add_plugins(
			DefaultPlugins
				.set(ImagePlugin::default_nearest())
				.set(WindowPlugin {
					window: WindowDescriptor {
						title: "gamejam".into(),
						width: H_WIN_SIZE,
						height: V_WIN_SIZE,
						resizable: false,
						fit_canvas_to_parent: true,
						..default()
					},
					..default()
				}),
		)
		.add_startup_system(create_cameras)
		.add_system(update)
		.add_plugin(CollisionPlugin)
		.add_plugin(PlayerPlugin)
		.add_plugin(TilemapPlugin)
		.add_plugin(CollectablesPlugin)
		.add_plugin(shaders::light::LightShaderPlugin)
		.run();
}
