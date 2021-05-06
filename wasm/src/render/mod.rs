use crate::programs;
use web_sys::WebGlRenderingContext as GL;

mod shader;

pub use self::shader::{Program, Shader};

pub struct Renderer {
    cell: programs::Cell,
    vao_ext: js_sys::Object,
}

impl Renderer {
    pub fn new(gl: &GL) -> Renderer {
        let vao_ext = gl
            .get_extension("OES_vertex_array_object")
            .expect("Get OES vao ext")
            .expect("OES vao ext");
        Renderer {
            cell: programs::Cell::new(gl),
            vao_ext,
        }
    }

    pub fn render(&mut self, gl: &GL, canvas_width: f32, canvas_height: f32) {
        self.cell.render(gl, canvas_width, canvas_height);
    }
}
