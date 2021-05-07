use crate::programs;
use na;
use web_sys::WebGlRenderingContext as GL;
use crate::fluid;

pub struct Camera {
    projection: [f32; 16],
}

pub struct Renderer {
    cell_map: fluid::CellMap,
    vao_ext: js_sys::Object
}

impl Renderer {
    pub fn new(gl: &GL) -> Renderer {
        let vao_ext = gl
            .get_extension("OES_vertex_array_object")
            .expect("Get OES vao ext")
            .expect("OES vao ext");
        Renderer {
            cell_map: fluid::CellMap::new(gl, 10),
            vao_ext,
        }
    }

    pub fn render(&mut self, gl: &GL, canvas_width: f32, canvas_height: f32) {

        let ratio = canvas_width / canvas_height;
        let proj = na::Orthographic3::new(-ratio, ratio, -1.0, 1.0, -1.0, 1.0);

        self.cell_map.render(gl, proj.as_matrix().as_slice());
    }
}
