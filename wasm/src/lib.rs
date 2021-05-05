extern crate wasm_bindgen;
extern crate nalgebra as na;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod common_funcs;
mod gl_setup;
mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct WASMClient {
    gl: GL,
    program_color_2d: programs::Color2D,
}

#[wasm_bindgen]
impl WASMClient {
    #[wasm_bindgen(constructor)]
    pub fn new(webgl_context: GL) -> Self {
        console_error_panic_hook::set_once();
        gl_setup::initialize_webgl_context(&webgl_context);
        Self {
            program_color_2d: programs::Color2D::new(&webgl_context),
            gl: webgl_context,
        }
    }

    pub fn update(&mut self, time: f32, width: f32, height: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, width, height);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(
            GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT,
        );

        let curr_state = app_state::get_curr_state();

        self.program_color_2d
            .render(&self.gl,
              curr_state.canvas_width ,
              curr_state.canvas_height);
    }
}
