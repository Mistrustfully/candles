#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

struct Lighting {
	radius: f32,
	time: f32,
	_pad: f32,
	_pad2: f32,
}
@group(1) @binding(2)
var<uniform> lighting: Lighting;

fn rand(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn light(light_center: vec2<f32>) -> f32 {
	let uv = coords_to_viewport_uv(light_center, view.viewport);
	let uv_centered = (uv * 2. - 1.0);
	let sin_time = sin(lighting.time + cos(lighting.time) * 2.0) / 25.0;

	// "Feathering" at the borders of the light, via random noise.
	// We change the seed every 10ms (I think?). We modulo that by two to limit it to two patters.
	let feathering = rand(uv_centered + round(lighting.time * 6.0) % 2.0) / 7.5;
    
	// Get the mask of the light
	let mask = (lighting.radius - feathering + sin_time) - distance(vec2<f32>(0.), uv_centered);

	// Set the alpha of the pixel to either 1 or 0 to keep it within the palette constrants.
	var alpha = mask;
	if alpha > 0.05 {
		alpha = 1.0;
	} else {
		alpha = 0.0;
	};

	return alpha;
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
	let uv = coords_to_viewport_uv(position.xy * vec2<f32>(view.viewport.z / view.viewport.w, 1.0), view.viewport);
	let uv_centered = (uv * 2. - 1.0);
    let original = textureSample(texture, our_sampler, uv);

	let light_alpha = light(position.xy * vec2<f32>(view.viewport.z / view.viewport.w, 1.0));
	let light2_alpha = light(vec2<f32>(position.x - position.y, position.y));

    return vec4<f32>(original.rgb, min(light_alpha + light2_alpha, 1.0));
}

