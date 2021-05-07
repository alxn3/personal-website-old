use crate::{app_state, programs::PixelMap};
use web_sys::WebGlRenderingContext as GL;

pub struct FluidSimulation {
  pixel_map: PixelMap
}

impl FluidSimulation {
    pub fn new(
        gl: &GL,
        width: usize,
        height: usize,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
    ) -> Self {
      let pixel_map =  PixelMap::new(gl, width, height, left, right, bottom, top);
      pixel_map.load_texture(gl);
      Self {
        pixel_map
      }
    }

    pub fn render(&self, gl: &GL, proj: &[f32]) {
      &self.pixel_map.render(gl, proj);
    }

    pub fn update(&mut self) {
      let delta_time = app_state::get_curr_state().time;
      for x in 0..self.pixel_map.get_width() {
        for y in 0..self.pixel_map.get_height() {
          &self.pixel_map.set_pixel(x, y,
            &[(f32::sin(delta_time / 1000.0 + x as f32 / self.pixel_map.get_width() as f32) * 256.0) as u8,
            (f32::cos(delta_time / 1000.0 + y as f32/ self.pixel_map.get_height() as f32) * 256.0) as u8,
            ((1.0 - f32::sin(delta_time / 1000.0 + x as f32/ self.pixel_map.get_width() as f32)) * 256.0) as u8 ,
            255
            ]
          );
        }
      }
    }
}
