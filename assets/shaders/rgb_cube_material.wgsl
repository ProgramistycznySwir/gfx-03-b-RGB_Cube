struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
	@location(0) world_pos: vec4<f32>,
	@location(1) world_normal: vec3<f32>,
	@location(2) uv: vec2<f32>
}

@fragment
fn fragment(
	input: VertexOutput
) -> @location(0) vec4<f32> {
	// return vec4<f32>(input.uv, 0.0, 0.0);
	return input.world_pos;
}