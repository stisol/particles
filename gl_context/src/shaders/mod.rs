//! Contains the shaders for rendering.

use std::{
    str,
    ops::Drop,
};

use Shader;
use Program;
use AbstractContext;
use Context;
use context::{UniformLocation, GLUint};
use std::mem;

/// Vertex shader for particles
pub const PARTICLES_VERTEX_SHADER: &[u8] = include_bytes!("particles_vertex.glslv");

/// Fragment shader for particles
pub const PARTICLES_FRAGMENT_SHADER: &[u8] = include_bytes!("particles_fragment.glslf");

/// Vertex shader for triangles
pub const TRIANGLES_VERTEX_SHADER: &[u8] = include_bytes!("triangles_vertex.glslv");

/// Fragment shader for triangles
pub const TRIANGLES_FRAGMENT_SHADER: &[u8] = include_bytes!("triangles_fragment.glslf");

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct ShaderAttribute {
    pub name: String,
    pub size: usize,
}

pub struct OurShader {
    pub program: Program,
    vs: Shader,
    fs: Shader,
    attributes: Vec<ShaderAttribute>,
    attribute_locations: Vec<GLUint>,
    attribute_size: usize,
}

impl OurShader {
    pub fn new(
        vertex_shader: &str,
        fragment_shader: &str,
        attributes: Vec<ShaderAttribute>) -> Self {

        let context = Context::get_context();
        // Compile vertex shader
        let vs = context
            .create_shader(ShaderType::Vertex)
            .expect("Failed to create vertex shader.");
        context.shader_source(&vs, vertex_shader);
        context.compile_shader(&vs);

        if let Some(log) = context.get_shader_info_log(&vs) {
            println!("vertex shader log: {}", log);
        }

        // Compile fragment shader
        let fs = context
            .create_shader(ShaderType::Fragment)
            .expect("Failed to create fragment shader.");
        context.shader_source(&fs, fragment_shader);
        context.compile_shader(&fs);

        if let Some(log) = context.get_shader_info_log(&fs) {
            println!("fragment shader log: {}", log);
        }

        // Link program
        let program = context
            .create_program()
            .expect("Failed to create shader program.");
        context.attach_shader(&program, &vs);
        context.attach_shader(&program, &fs);
        context.link_program(&program);

        let mut attribute_locations : Vec<GLUint> = Vec::new();
        context.use_program(&program);
        let mut attribute_size = 0;
        let mut index = 0;
        
        for attrib in &attributes {
            context.bind_attrib_location(&program, index, &attrib.name);
            //let attrib_loc : i32 = context.get_attrib_location(&program, &attrib.name) as i32;
            let attrib_loc : i32 = index as i32;
            attribute_locations.push(attrib_loc as GLUint);
            attribute_size += attrib.size;
            index = index + 1;
        }

        OurShader {
            program,
            fs,
            vs,
            attributes,
            attribute_locations,
            attribute_size,
        }
    }

    pub fn set_active(&self) {
        let context = Context::get_context();
        context.use_program(&self.program);

        let mut offset : usize = 0;
        for i in 0..self.attributes.len() {
            let attrib = &self.attributes[i];
            let attrib_pos = self.attribute_locations[i];
            let float_size = mem::size_of::<i32>() as i32; 
            let off = offset as i32;
            context.vertex_attrib_pointer(&attrib_pos, attrib.size as i32, Context::FLOAT, false, self.attribute_size as i32, off);
            context.enable_vertex_attrib_array(&attrib_pos);
            offset += attrib.size;
        }
    }

    pub fn get_uniform_location(&self) -> UniformLocation {
        Context::get_context().get_uniform_location(&self.program, "MVP")
    }
}

impl Drop for OurShader {
    fn drop(&mut self) {
        let context = Context::get_context();
        context.delete_program(&self.program);
        context.delete_shader(&self.vs);
        context.delete_shader(&self.fs);
    }
}
