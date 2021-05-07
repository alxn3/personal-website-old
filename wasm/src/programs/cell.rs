use super::super::render;
use crate::util;
use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Cell {
    shader: render::Shader,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
    vertices: [f32; 8],
    indices: [u16; 6],
}

impl Cell {
    pub fn new(gl: &GL, left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let shader = render::Shader::from_shaders(gl,
          crate::shaders::vertex::CELL,
          crate::shaders::fragment::CELL
        ).unwrap();

        let vertices_rect: [f32; 8] = [left, top, left, bottom, right, top, right, bottom];
        let indices_rect: [u16; 6] = [0, 1, 2, 2, 1, 3];

        Self {
            u_color: gl.get_uniform_location(&shader.program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&shader.program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&shader.program, "uTransform").unwrap(),
            vertices: vertices_rect,
            indices: indices_rect,
            shader,
        }
    }

    fn buffer_attributes(&self, gl: &GL) {
      let pos_attrib = gl.get_attrib_location(&self.shader.program, "aPosition");
      gl.enable_vertex_attrib_array(pos_attrib as u32);

      util::gl::buffer_f32_data(gl, &self.vertices, pos_attrib as u32, 2);
      util::gl::buffer_u16_indices(gl, &self.indices);
    }

    pub fn render(&self, gl: &GL, proj: &[f32], r: f32, g: f32, b: f32) {
        gl.use_program(Some(&self.shader.program));

        self.buffer_attributes(gl);

        gl.uniform4f(Some(&self.u_color), r, g, b, 1.0);
        gl.uniform1f(Some(&self.u_opacity), 1.0);

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &proj);

        gl.draw_elements_with_i32(GL::TRIANGLES, self.indices.len() as i32, GL::UNSIGNED_SHORT, 0);
    }
}
