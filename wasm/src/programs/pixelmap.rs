use crate::{render, util};
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct PixelMap {
    shader: render::Shader,
    framebuffer: WebGlFramebuffer,
    texture: WebGlTexture,
    pixels: Box<[u8]>,
    width: usize,
    height: usize,
    vertices: [f32; 8],
    indices: [u16; 6],
    tex_coord: [f32; 8],
}

impl PixelMap {
    pub fn new(
        gl: &GL,
        width: usize,
        height: usize,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
    ) -> Self {
        let shader = render::Shader::from_shaders(
            gl,
            crate::shaders::vertex::PIXEL_MAP,
            crate::shaders::fragment::PIXEL_MAP,
        )
        .unwrap();

        let vertices: [f32; 8] = [left, top, left, bottom, right, top, right, bottom];
        let indices: [u16; 6] = [0, 1, 2, 2, 1, 3];

        let mut vec: Vec<u8> = Vec::new();
        // for i in 0..(width * height) {
        //   vec.push((i % 256) as u8);
        //   vec.push(((i + 128) % 256) as u8);
        //   vec.push((255 - i % 256) as u8);
        //   vec.push((255) as u8);
        // }
        for x in 0..width {
            for y in 0..height {
                vec.push((x % 256) as u8);
                vec.push((y % 256) as u8);
                vec.push((255 - x % 256) as u8);
                vec.push(255);
            }
        }

        let pixels: Box<[u8]> = vec.into_boxed_slice();

        let texture = gl.create_texture().expect("Cannot create gl texture");
        let framebuffer = gl
            .create_framebuffer()
            .expect("Cannot create gl frame buffer");

        Self {
            shader,
            texture,
            pixels,
            width,
            height,
            framebuffer,
            vertices,
            indices,
            tex_coord: [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn load_texture(&self, gl: &GL) {
        gl.bind_texture(GL::TEXTURE_2D, Some(&self.texture));
        // gl.pixel_store_i(GL::UNPACK_ALIGNMENT, 1);

        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            self.width as i32,
            self.height as i32,
            0,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            Some(&self.pixels),
        );

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

        // gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&self.framebuffer));
        // gl.framebuffer_texture_2d(GL::FRAMEBUFFER, GL::COLOR_ATTACHMENT0, GL::TEXTURE_2D, Some(&self.texture), 0);
    }

    fn buffer_attributes(&self, gl: &GL) {
        let pos_attrib = gl.get_attrib_location(&self.shader.program, "aVertexPosition");
        gl.enable_vertex_attrib_array(pos_attrib as u32);
        let tex_attrib = gl.get_attrib_location(&self.shader.program, "aTextureCoord");
        gl.enable_vertex_attrib_array(tex_attrib as u32);

        util::gl::buffer_f32_data(gl, &self.vertices, pos_attrib as u32, 2);
        util::gl::buffer_f32_data(gl, &self.tex_coord, tex_attrib as u32, 2);
        util::gl::buffer_u16_indices(gl, &self.indices);
    }

    pub fn render(&self, gl: &GL, proj: &[f32]) {
        gl.use_program(Some(&self.shader.program));

        self.buffer_attributes(gl);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
          GL::TEXTURE_2D,
          0,
          GL::RGBA as i32,
          self.width as i32,
          self.height as i32,
          0,
          GL::RGBA,
          GL::UNSIGNED_BYTE,
          Some(&self.pixels),
      );

        gl.uniform_matrix4fv_with_f32_array(
            self.shader
                .get_uniform_location(gl, "uProjectionMatrix")
                .as_ref(),
            false,
            &proj,
        );
        gl.uniform1i(self.shader.get_uniform_location(gl, "uSampler").as_ref(), 0);

        gl.draw_elements_with_i32(
            GL::TRIANGLES,
            self.indices.len() as i32,
            GL::UNSIGNED_SHORT,
            0,
        );
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> [u8; 4] {
        let index = 4 * (y * self.height + x);
        [
            self.pixels[index],
            self.pixels[index + 1],
            self.pixels[index + 2],
            self.pixels[index + 3],
        ]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, new_pixel: &[u8; 4]) {
      let index = 4 * (y * self.height + x);
      self.pixels[index + 1] = new_pixel[1];
      self.pixels[index + 3] = new_pixel[3];
      self.pixels[index + 2] = new_pixel[2];
      self.pixels[index] = new_pixel[0];
    }

    pub fn get_width(&self) -> usize {
      self.width
    }

    pub fn get_height(&self) -> usize {
      self.height
    }
}
