use web_sys::WebGlRenderingContext;

pub fn initialize_webgl_context(gl: &WebGlRenderingContext) {
    gl.enable(WebGlRenderingContext::BLEND);
    gl.blend_func(WebGlRenderingContext::SRC_ALPHA, WebGlRenderingContext::ONE_MINUS_SRC_ALPHA);
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.0);
}
