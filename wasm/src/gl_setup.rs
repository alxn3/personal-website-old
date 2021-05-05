use web_sys::WebGlRenderingContext as GL;

pub fn initialize_webgl_context(gl: &GL) {
    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.0);
}
