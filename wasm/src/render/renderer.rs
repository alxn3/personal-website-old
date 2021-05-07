use crate::programs;
use na;
use web_sys::WebGlRenderingContext as GL;
use crate::fluid;

pub struct Camera {
    projection: [f32; 16],
}

pub struct Renderer {
    pixel_map: programs::PixelMap,
    vao_ext: js_sys::Object
}

impl Renderer {
    pub fn new(gl: &GL) -> Renderer {
        let vao_ext = gl
            .get_extension("OES_vertex_array_object")
            .expect("Get OES vao ext")
            .expect("OES vao ext");
        let pixel_map = programs::PixelMap::new(gl, 20, 20, -1.0, 1.0, -1.0, 1.0);
        pixel_map.load_texture(gl);
        Renderer {
            pixel_map,
            vao_ext,
        }
    }

    pub fn render(&mut self, gl: &GL, canvas_width: f32, canvas_height: f32) {

        let ratio = canvas_width / canvas_height;
        let proj = na::Orthographic3::new(-ratio, ratio, -1.0, 1.0, -1.0, 1.0);

        self.pixel_map.render(gl, proj.as_matrix().as_slice());
    }

    pub fn update(&mut self) {
      &self.pixel_map.update();
    }
}
