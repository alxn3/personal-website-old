use std::{cell::RefCell, collections::HashMap};

use web_sys::{WebGlProgram, WebGlShader};
use web_sys::{WebGlRenderingContext as GL, WebGlUniformLocation};

pub struct Shader {
    gl: GL,
    pub program: WebGlProgram,
    uniforms: RefCell<HashMap<String, WebGlUniformLocation>>,
}

impl Shader {
    pub fn from_shaders(gl: &GL, vert_shader: &str, frag_shader: &str) -> Result<Shader, String> {

        let vert_shader = compile_shader(gl, GL::VERTEX_SHADER, vert_shader)?;
        let frag_shader = compile_shader(gl, GL::FRAGMENT_SHADER, frag_shader)?;

        let program = link_program(gl, &vert_shader, &frag_shader)?;

        if gl
            .get_program_parameter(&program, GL::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            let uniforms = RefCell::new(HashMap::new());
            Ok(Shader {
                gl: gl.clone(),
                program,
                uniforms,
            })
        } else {
            Err(gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

    pub fn get_uniform_location(
        &self,
        gl: &GL,
        uniform_name: &str,
    ) -> Option<WebGlUniformLocation> {
        let mut uniforms = self.uniforms.borrow_mut();

        if uniforms.get(uniform_name).is_none() {
            uniforms.insert(
                uniform_name.to_string(),
                gl.get_uniform_location(&self.program, uniform_name)
                    .expect(&format!(r#"Uniform '{}' not found"#, uniform_name)),
            );
        }

        Some(uniforms.get(uniform_name).expect("loc").clone())
    }

    pub fn set_used(&self) {
        self.gl.use_program(Some(&self.program));
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program));
    }
}

pub fn link_program(
  gl: &GL,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = gl
      .create_program()
      .ok_or_else(|| String::from("Unable to create shader object"))?;

  gl.attach_shader(&program, vert_shader);
  gl.attach_shader(&program, frag_shader);
  gl.link_program(&program);

  if gl
      .get_program_parameter(&program, GL::LINK_STATUS)
      .as_bool()
      .unwrap_or(false)
  {
      Ok(program)
  } else {
      Err(gl
          .get_program_info_log(&program)
          .unwrap_or_else(|| String::from("Unknown error creating program object")))
  }
}

pub fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}
