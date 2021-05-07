extern crate nalgebra as na;
extern crate wasm_bindgen;
use render::Renderer;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod gl_setup;
mod programs;
mod render;
mod shaders;
mod fluid;
mod util;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct WASMClient {
    renderer: Renderer,
    gl: Rc<GL>,
}

#[wasm_bindgen]
impl WASMClient {
    #[wasm_bindgen(constructor)]
    pub fn new(webgl_context: GL) -> Self {
        console_error_panic_hook::set_once();
        gl_setup::initialize_webgl_context(&webgl_context);

        let gl = Rc::new(webgl_context);
        let renderer = Renderer::new(&gl);

        Self { gl, renderer }
    }

    pub fn update(&mut self, time: f32, width: f32, height: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, width, height);
        &self.renderer.update();
        Ok(())
    }

    pub fn render(&mut self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = app_state::get_curr_state();

        &self
            .renderer
            .render(&self.gl, curr_state.canvas_width, curr_state.canvas_height);
    }
}
