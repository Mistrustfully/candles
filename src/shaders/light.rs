use bevy::{
	prelude::*,
	reflect::TypeUuid,
	render::{
		render_resource::{AsBindGroup, Extent3d, ShaderRef, ShaderType},
		view::RenderLayers,
	},
	sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

use crate::{MainRenderTarget, ShouldResize};

#[derive(Resource, Clone, ShaderType, Copy)]
pub struct LightingConfig {
	pub radius: f32,
	pub time: f32,
	_pad: f32, // Welcome to the world of WASM where if you don't pad things to 16 bytes it shits
	_pad2: f32, // itself.
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct LightingMaterial {
	/// In this example, this image will be the result of the main camera.
	#[texture(0)]
	#[sampler(1)]
	pub source_image: Handle<Image>,
	#[uniform(2)]
	pub lighting_config: LightingConfig,
}

impl Material2d for LightingMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/light.wgsl".into()
	}
}

pub fn init_lighting_shader(
	mut commands: Commands,
	windows: ResMut<Windows>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut post_processing_materials: ResMut<Assets<LightingMaterial>>,
	render_target: Res<MainRenderTarget>,
) {
	let window = windows.get_primary().unwrap();
	let size = Extent3d {
		width: window.physical_width(),
		height: window.physical_height(),
		..default()
	};

	let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

	let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
		size.width as f32,
		size.height as f32,
	))));

	// This material has the texture that has been rendered.
	let material_handle = post_processing_materials.add(LightingMaterial {
		source_image: render_target.0.clone_weak(),
		lighting_config: LightingConfig {
			radius: 1.0,
			time: 0.0,
			_pad: 0.0,
			_pad2: 0.0,
		},
	});

	// Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
	commands.spawn((
		MaterialMesh2dBundle {
			mesh: quad_handle.into(),
			material: material_handle,
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 1.5),
				..default()
			},
			..default()
		},
		ShouldResize,
		post_processing_pass_layer,
	));

	// The post-processing pass camera.
	commands.spawn((
		Camera2dBundle {
			camera: Camera {
				// renders after the first main camera which has default value: 0.
				priority: 1,
				..default()
			},
			..Camera2dBundle::default()
		},
		post_processing_pass_layer,
	));
}

pub fn update_lighting_shader(
	mut lighting_material: ResMut<Assets<LightingMaterial>>,
	lighting_config: Res<LightingConfig>,
	time: Res<Time>,
) {
	for mut lighting in lighting_material.iter_mut() {
		lighting.1.lighting_config.radius = lighting_config.radius;
		lighting.1.lighting_config.time = time.raw_elapsed_seconds();
	}
}

fn update_lighting_config(mut lighting_config: ResMut<LightingConfig>, time: Res<Time>) {
	lighting_config.radius = 0.5;
}

pub struct LightShaderPlugin;
impl Plugin for LightShaderPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(Material2dPlugin::<LightingMaterial>::default())
			.add_startup_system_to_stage(StartupStage::PostStartup, init_lighting_shader)
			.add_system(update_lighting_shader)
			.add_system(update_lighting_config)
			.insert_resource(LightingConfig {
				radius: 1.0,
				time: 0.0,
				_pad: 0.0,
				_pad2: 0.0,
			});
	}
}
