use super::super::render_gl as cf;
use super::super::render_gl::*;
use js_sys::WebAssembly;
use na;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Color2D {
    program: Program,
    index_count: i32,
    rect_vertice_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2D {
    pub fn new(gl: &GL) -> Self {
        let program = Program::from_shaders(
            gl,
            &[
                Shader::from_vert_source(gl, super::super::shaders::vertex::COLOR_2D).unwrap(),
                Shader::from_frag_source(gl, super::super::shaders::fragment::COLOR_2D).unwrap(),
            ],
        ).unwrap();

        let vertices_rect: [f32; 8] = [-2.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5, -0.5];

        let indices_rect: [u16; 6] = [0, 1, 2, 2, 1, 3];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vertices_location = vertices_rect.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + vertices_rect.len() as u32,
        );
        let buffer_rect = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let indices_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let indices_location = indices_rect.as_ptr() as u32 / 2;
        let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
            indices_location,
            indices_location + indices_rect.len() as u32,
        );
        let buffer_indices = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            GL::STATIC_DRAW,
        );

        Self {
            u_color: gl.get_uniform_location(&program.id, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program.id, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program.id, "uTransform").unwrap(),
            index_count: indices_array.length() as i32,
            rect_vertice_buffer: buffer_rect,
            program: program,
        }
    }

    pub fn render(&self, gl: &GL, canvas_width: f32, canvas_height: f32) {
        gl.use_program(Some(&self.program.id));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(Some(&self.u_color), 0.0, 0.5, 0.5, 1.0);

        gl.uniform1f(Some(&self.u_opacity), 1.0);

        let ratio = canvas_width / canvas_height;

        let proj = na::Orthographic3::new(-ratio, ratio, -1.0, 1.0, -1.0, 1.0);
        //log(&ratio.to_string());

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_transform),
            false,
            &proj.as_matrix().as_slice(),
        );

        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
    }
}
