use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlProgram, WebGlShader};

pub struct Program {
    gl: GL,
    pub id: WebGlProgram,
}

impl Program {
    pub fn from_shaders(gl: &GL, shaders: &[Shader]) -> Result<Program, String> {
        let program = gl
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        for shader in shaders {
            gl.attach_shader(&program, &shader.id);
        }
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, GL::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            for shader in shaders {
                gl.detach_shader(&program, &shader.id);
            }
            Ok(Program {
                gl: gl.clone(),
                id: program,
            })
        } else {
            Err(gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

    pub fn set_used(&self) {
        self.gl.use_program(Some(&self.id));
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.id));
    }
}
pub struct Shader {
    gl: GL,
    pub id: WebGlShader,
}

impl Shader {
    pub fn from_source(gl: &GL, source: &str, kind: u32) -> Result<Shader, String> {
        let id = compile_shader(gl, kind, source)?;
        Ok(Shader { gl: gl.clone(), id })
    }

    pub fn from_vert_source(gl: &GL, source: &str) -> Result<Shader, String> {
        Shader::from_source(gl, source, GL::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: &GL, source: &str) -> Result<Shader, String> {
        Shader::from_source(gl, source, GL::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.delete_shader(Some(&self.id));
    }
}

pub fn compile_shader(context: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}
