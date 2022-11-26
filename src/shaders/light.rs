use bevy::{
	math::{Vec3Swizzles, Vec4Swizzles},
	prelude::*,
	reflect::TypeUuid,
	render::{
		render_resource::{AsBindGroup, Extent3d, ShaderRef, ShaderType},
		view::RenderLayers,
	},
	sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

use crate::{player::PlayerCamera, MainRenderTarget, ShouldResize};

#[derive(Resource, Debug)]
pub struct Lights(pub [Vec4; 16]);

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct LightingMaterial {
	/// In this example, this image will be the result of the main camera.
	#[texture(0)]
	#[sampler(1)]
	pub source_image: Handle<Image>,
	#[uniform(2)]
	pub lights: [Vec4; 16],
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
		lights: [Vec4::ZERO; 16],
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
	lights: Res<Lights>,
	camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
	time: Res<Time>,
) {
	let (camera, transform) = camera_query.get_single().unwrap();
	for mut lighting in lighting_material.iter_mut() {
		for (i, v) in lights.0.iter().enumerate() {
			let vec_position =
				camera.world_to_ndc(transform, v.xyz()).unwrap() * Vec3::new(1.0, -1.0, 1.0);

			lighting.1.lights[i] = Vec4::new(
				vec_position.x,
				vec_position.y,
				time.raw_elapsed_seconds(),
				v.w,
			);
		}
	}
}

pub struct LightShaderPlugin;
impl Plugin for LightShaderPlugin {
	fn build(&self, app: &mut App) {
		let mut lights = [Vec4::ZERO; 16];
		for i in 0..15 {
			lights[i].w = (i as f32) / 16.0;
			lights[i].x = (i as f32) * 64.0;
			lights[i].y = 128.0
		}

		app.add_plugin(Material2dPlugin::<LightingMaterial>::default())
			.add_startup_system_to_stage(StartupStage::PostStartup, init_lighting_shader)
			.add_system(update_lighting_shader)
			.insert_resource(Lights(lights));
	}
}
