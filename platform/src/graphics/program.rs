use glow::HasContext;

use crate::graphics::shader::ShaderBuilder;

pub struct Program {
    pub handle: glow::Program,
    pub shader_handles: Vec<glow::Shader>,
    // pub uniform_loc_cache: HashMap<String, glow::UniformLocation>,
    pub shaders_path: &'static str,
}

impl Program {
    pub fn new(gl: &glow::Context) -> Self {
        let gl_prog = unsafe { gl.create_program().unwrap() };

        Self {
            handle: gl_prog,
            shader_handles: Vec::new(),
            // uniform_loc_cache: HashMap::new(),
            shaders_path: "",
        }
    }

    pub fn from_path(gl: &glow::Context, shaders_path: &'static str) -> Self {
        let gl_prog = unsafe { gl.create_program().unwrap() };

        let mut prog = Self {
            handle: gl_prog,
            shader_handles: Vec::new(),
            // uniform_loc_cache: HashMap::new(),
            shaders_path,
        };
        prog.add_shaders(gl, shaders_path);

        prog
    }

    pub fn reload_shaders(&mut self, gl: &glow::Context) {
        self.shader_handles.clear();
        // self.uniform_loc_cache.clear();
        unsafe { gl.delete_program(self.handle) };

        let gl_prog = unsafe { gl.create_program().unwrap() };
        self.handle = gl_prog;
        self.add_shaders(gl, self.shaders_path);
    }

    /// Accepts a single string containing multiple shader definitions, each prefaced by\
    /// #shader *<shader_type>*\
    /// where *<shader_type>* can be vertex, fragment, and so on.
    pub fn add_shaders(&mut self, gl: &glow::Context, shaders_path: &'static str) {
        self.shaders_path = shaders_path;

        match ShaderBuilder::build(shaders_path) {
            Ok(shaders) => {
                for (shader_type, shader_source) in shaders.iter() {
                    self.add_shader(gl, *shader_type, shader_source);
                }
            }
            Err(error) => {
                println!("{}", error)
            }
        }

        for shader in self.shader_handles.iter() {
            unsafe { gl.detach_shader(self.handle, *shader) }
        }
    }

    /// Attempts to compile and link the shader to this program
    fn add_shader(&mut self, gl: &glow::Context, shader_type: u32, shader_source: &str) {
        unsafe {
            // This 'NativeShader' type is a u32 that represents the pointer to our new shader object
            let shader = gl.create_shader(shader_type).unwrap();

            // We associate the shader object with a source code string
            gl.shader_source(shader, shader_source);

            println!(
                "{}",
                format!(
                    "Adding {} Shader from '{}'",
                    if shader_type == glow::VERTEX_SHADER {
                        "Vertex"
                    } else if shader_type == glow::FRAGMENT_SHADER {
                        "Fragment"
                    } else if shader_type == glow::GEOMETRY_SHADER {
                        "Geometry"
                    } else if shader_type == glow::COMPUTE_SHADER {
                        "Compute"
                    } else {
                        "Unknown" // TODO: Add more types
                    },
                    self.shaders_path,
                )
            );
            self.compile_shader(gl, shader);
            self.link_shader(gl, shader);

            self.shader_handles.push(shader);
        }
    }

    fn compile_shader(&self, gl: &glow::Context, shader: glow::Shader) {
        unsafe {
            // Compiles the source code strings that have been stored in the shader object
            gl.compile_shader(shader);
        }

        self.print_shader_compile_status(gl, shader);
        self.print_shader_info_log(gl, shader);
    }

    fn link_shader(&self, gl: &glow::Context, shader: glow::Shader) {
        unsafe {
            // We associate the shader object with a source code string
            gl.attach_shader(self.handle, shader);

            // This uses the attached shader objects to create a single executable to run on the GPU
            gl.link_program(self.handle);

            // If a shader object to be deleted is attached to a program object, it will be flagged for deletion, but
            // it will not be deleted until it is no longer attached to any program object
            gl.delete_shader(shader);
        }

        self.print_program_link_status(gl);
        self.print_program_info_log(gl);
    }

    /// Prints the information log for the specified shader object.\
    /// The information log for a shader object is modified when the shader is compiled.
    fn print_shader_info_log(&self, gl: &glow::Context, shader: glow::Shader) {
        let msg = unsafe { gl.get_shader_info_log(shader) };
        let msg = msg.trim();

        println!(
            "{}{}{}",
            "Program Info Log:",
            if msg.is_empty() { "" } else { "/n" },
            msg
        );
    }

    fn print_shader_compile_status(&self, gl: &glow::Context, shader: glow::Shader) {
        println!("{}{}", "Shader Compile Status: ", unsafe {
            gl.get_shader_compile_status(shader)
        })
    }

    fn print_program_info_log(&self, gl: &glow::Context) {
        let msg = unsafe { gl.get_program_info_log(self.handle) };
        let msg = msg.trim();

        println!(
            "{}{}{}",
            "Program Info Log:",
            if msg.is_empty() { "" } else { "/n" },
            msg
        );
    }

    fn print_program_link_status(&self, gl: &glow::Context) {
        println!("{}{}", "Program Link Status: ", unsafe {
            gl.get_program_link_status(self.handle)
        });
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.handle));
        }
    }

    pub fn unbind(gl: &glow::Context) {
        unsafe {
            gl.use_program(None);
        }
    }

    pub fn delete(&self, gl: &glow::Context) {
        Program::unbind(gl);
        unsafe {
            gl.delete_program(self.handle);
        }
    }

    // pub fn set_uniform(&mut self, gl: &glow::Context, name: String, data_stype: ShaderData) {
    //     let loc = if let Some(location) = self.uniform_loc_cache.get(&name) {
    //         *location
    //     } else if let Some(location) = unsafe { gl.get_uniform_location(self.handle, &name) } {
    //         self.uniform_loc_cache.insert(name, location);
    //         location
    //     } else {
    //         println!(
    //             "Attempted to set uniform '{}', but it cannot be found in the current program",
    //             name
    //         );
    //         return;
    //     };

    //     match data_stype {
    //         ShaderData::Uint1(x) => unsafe { gl.uniform_1_u32(Some(&loc), x) },
    //         ShaderData::Uint2(x, y) => unsafe { gl.uniform_2_u32(Some(&loc), x, y) },
    //         ShaderData::Uint3(x, y, z) => unsafe { gl.uniform_3_u32(Some(&loc), x, y, z) },
    //         ShaderData::Uint4(x, y, z, w) => unsafe { gl.uniform_4_u32(Some(&loc), x, y, z, w) },
    //         ShaderData::Int1(x) => unsafe { gl.uniform_1_i32(Some(&loc), x) },
    //         ShaderData::Int2(x, y) => unsafe { gl.uniform_2_i32(Some(&loc), x, y) },
    //         ShaderData::Int3(x, y, z) => unsafe { gl.uniform_3_i32(Some(&loc), x, y, z) },
    //         ShaderData::Int4(x, y, z, w) => unsafe { gl.uniform_4_i32(Some(&loc), x, y, z, w) },
    //         ShaderData::Float1(x) => unsafe { gl.uniform_1_f32(Some(&loc), x) },
    //         ShaderData::Float2(x, y) => unsafe { gl.uniform_2_f32(Some(&loc), x, y) },
    //         ShaderData::Float3(x, y, z) => unsafe { gl.uniform_3_f32(Some(&loc), x, y, z) },
    //         ShaderData::Float4(x, y, z, w) => unsafe { gl.uniform_4_f32(Some(&loc), x, y, z, w) },
    //         ShaderData::Mat2f() => todo!(),
    //         ShaderData::Mat3f() => todo!(),
    //         ShaderData::Mat4f() => todo!(),
    //     }
    // }
}
