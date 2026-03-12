struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.tex_coords = model.tex_coords;
    // Positions are expected to already be in clip-space coordinates.
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment Shader Inputs
@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;

fn srgb_channel_to_linear(c: f32) -> f32 {
    if (c <= 0.04045) {
        return c / 12.92;
    }
    return pow((c + 0.055) / 1.055, 2.4);
}

fn srgb_to_linear(rgb: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        srgb_channel_to_linear(rgb.r),
        srgb_channel_to_linear(rgb.g),
        srgb_channel_to_linear(rgb.b)
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample texture
    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // Vertex colors are authored in sRGB-like UI space; convert to linear to match
    // texture sampling/blending in a linear workflow.
    let tint_linear = vec4<f32>(srgb_to_linear(in.color.rgb), in.color.a);

    // Multiply texture color by vertex color (tint)
    return tex_color * tint_linear;
}