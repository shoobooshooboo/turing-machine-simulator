// This shader computes the chromatic aberration effect

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct PostProcessSettings {
    intensity: f32,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

struct TimeData{
    time: f32,
}
@group(0) @binding(3) var<uniform> time: TimeData;

@fragment
fn fragment(
    in: FullscreenVertexOutput
) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let position = in.position;
    let offset_strength = settings.intensity;
    let screen_height = 800.0;
    let big_line_width = 10.0;
    let period = 6.0;
    let base_dim = 0.25;

    let t = time.time % period;
    let big_line_location = screen_height * t / period;
    var big_lines = clamp(abs(position.y % screen_height - big_line_location), 0, big_line_width) / big_line_width + base_dim;
    let little_lines =  f32(u32(position.y) % 3u);
    let scan_effect = little_lines * big_lines;

    return vec4<f32>(
        textureSample(screen_texture, texture_sampler, uv + vec2<f32>(-offset_strength * 3.0, 0.0)).r * scan_effect,
        textureSample(screen_texture, texture_sampler, uv + vec2<f32>(-offset_strength * 2.0, 0.0)).g * scan_effect,
        textureSample(screen_texture, texture_sampler, uv + vec2<f32>(-offset_strength, 0.0)).b * scan_effect,
        1.0
    );
}

fn clamp(x: f32, min_val: f32, max_val: f32) -> f32{
    return min(max(x, min_val), max_val);
}