pub enum ShaderDataType {
    Uint1,
    Uint2,
    Uint3,
    Uint4,
    Int1,
    Int2,
    Int3,
    Int4,
    Float1,
    Float2,
    Float3,
    Float4,
    Mat2f,
    Mat3f,
    Mat4f,
}

impl ShaderDataType {
    pub fn element_count(&self) -> u32 {
        match self {
            ShaderDataType::Uint1 => 1,
            ShaderDataType::Uint2 => 2,
            ShaderDataType::Uint3 => 3,
            ShaderDataType::Uint4 => 4,
            ShaderDataType::Int1 => 1,
            ShaderDataType::Int2 => 2,
            ShaderDataType::Int3 => 3,
            ShaderDataType::Int4 => 4,
            ShaderDataType::Float1 => 1,
            ShaderDataType::Float2 => 2,
            ShaderDataType::Float3 => 3,
            ShaderDataType::Float4 => 4,
            ShaderDataType::Mat2f => 4,
            ShaderDataType::Mat3f => 9,
            ShaderDataType::Mat4f => 16,
        }
    }

    pub fn size_bytes(&self) -> u32 {
        match self {
            ShaderDataType::Uint1 => 1 * 4,
            ShaderDataType::Uint2 => 2 * 4,
            ShaderDataType::Uint3 => 3 * 4,
            ShaderDataType::Uint4 => 4 * 4,
            ShaderDataType::Int1 => 1 * 4,
            ShaderDataType::Int2 => 2 * 4,
            ShaderDataType::Int3 => 3 * 4,
            ShaderDataType::Int4 => 4 * 4,
            ShaderDataType::Float1 => 1 * 4,
            ShaderDataType::Float2 => 2 * 4,
            ShaderDataType::Float3 => 3 * 4,
            ShaderDataType::Float4 => 4 * 4,
            ShaderDataType::Mat2f => 4 * 4,
            ShaderDataType::Mat3f => 9 * 4,
            ShaderDataType::Mat4f => 16 * 4,
        }
    }

    pub fn gl_type(&self) -> u32 {
        match self {
            ShaderDataType::Uint1
            | ShaderDataType::Uint2
            | ShaderDataType::Uint3
            | ShaderDataType::Uint4 => glow::UNSIGNED_INT,
            ShaderDataType::Int1
            | ShaderDataType::Int2
            | ShaderDataType::Int3
            | ShaderDataType::Int4 => glow::INT,
            ShaderDataType::Float1
            | ShaderDataType::Float2
            | ShaderDataType::Float3
            | ShaderDataType::Float4
            | ShaderDataType::Mat2f
            | ShaderDataType::Mat3f
            | ShaderDataType::Mat4f => glow::FLOAT,
        }
    }
}

pub enum ShaderData {
    Uint1(u32),
    Uint2(u32, u32),
    Uint3(u32, u32, u32),
    Uint4(u32, u32, u32, u32),
    Int1(i32),
    Int2(i32, i32),
    Int3(i32, i32, i32),
    Int4(i32, i32, i32, i32),
    Float1(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Float4(f32, f32, f32, f32),
    Mat2f(),
    Mat3f(),
    Mat4f(),
}

pub struct ShaderBuilder {
    entrypoint: &'static str,
    includes: Vec<String>,
    read_string: String,
    write_string: String,
}

impl ShaderBuilder {
    pub fn build(entrypoint: &'static str) -> Result<Vec<(u32, String)>, String> {
        let shaders_source = match std::fs::read_to_string(entrypoint) {
            Ok(ss) => ss,
            Err(error) => return Err(error.to_string()),
        };

        let mut sb = ShaderBuilder {
            entrypoint,
            includes: Vec::new(),
            read_string: String::new(),
            write_string: String::new(),
        };
        let mut shaders_final = Vec::new();

        for shader_source in shaders_source.split("#shader ") {
            let (shader_type, shader_source_no_type) = match shader_source.split_once("\r") {
                Some(("vertex", s)) => (glow::VERTEX_SHADER, s),
                Some(("fragment", s)) => (glow::FRAGMENT_SHADER, s),
                Some(("geometry", s)) => (glow::GEOMETRY_SHADER, s),
                Some(("compute", s)) => (glow::COMPUTE_SHADER, s),
                Some((st, _)) => {
                    return Err(format!("Unknown Shader Type Declared '{st}'").to_string());
                }
                _ => continue,
            };

            shaders_final.push((shader_type, sb.resolve_includes(shader_source_no_type)));
        }

        Ok(shaders_final)
    }

    fn resolve_includes(&mut self, shader_source: &str) -> String {
        self.includes.clear();

        self.read_string = shader_source.to_owned();
        self.write_string.clear();

        let mut finished = false;

        while !finished {
            let mut iter = self.read_string.lines().peekable();
            while let Some(line) = iter.next() {
                if line.trim().starts_with("#include ") {
                    let path = line.split('"').nth(1).unwrap();
                    let include_directive = format!(r#"#include "{path}""#);

                    // if we've seen this path before, skip
                    if self.includes.iter().any(|s| s == path) {
                        self.write_string = self.read_string.replace(&include_directive, "");
                        break;
                    } else {
                        self.includes.push(path.to_owned())
                    }

                    // replace directive with the file if found
                    if let Ok(included_source) = std::fs::read_to_string(path) {
                        self.write_string = self
                            .read_string
                            .replace(&include_directive, &included_source);
                        break;
                    } else {
                        println!(
                            "Unable to include '{}' in shader '{}'",
                            path, self.entrypoint
                        );
                        self.write_string = self.read_string.replace(&include_directive, "");
                    }
                }

                if iter.peek().is_none() {
                    finished = true;
                }
            }

            // swap the iterated string with the modified one
            std::mem::swap(&mut self.read_string, &mut self.write_string);
        }

        self.write_string.clone()
    }
}
