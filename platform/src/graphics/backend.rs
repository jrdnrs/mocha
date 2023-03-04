use crate::graphics::{
    buffer::{BufferElement, BufferLayout, BufferUsage, VertexArray},
    maths::Mat4f,
    program::Program,
    shader::ShaderDataType,
};
use glow::HasContext;

const QUAD_SIZE: u32 = std::mem::size_of::<mocha_core::Quad>() as u32;
const MAX_QUADS: u32 = 1_000;
const QUAD_BUFFER_SIZE: u32 = QUAD_SIZE * MAX_QUADS;
// const MAX_VERTICES: u32 = MAX_QUADS * 4;
// const MAX_INDICES: u32 = MAX_QUADS * 6;
// const INDEX_BUFFER_SIZE: u32 = 4 * MAX_INDICES;

pub struct QuadPipeline {
    pub program: Program,
    vertex_array: VertexArray,
    transform_loc: glow::UniformLocation,

    buffer: Vec<mocha_core::Quad>,
}

impl QuadPipeline {
    pub fn new(gl: &glow::Context) -> Self {
        let program = Program::from_path(gl, "platform/shaders/quad.glsl");
        let transform_loc = unsafe {
            gl.get_uniform_location(program.handle, "u_projection")
                .unwrap()
        };
        let vertex_array = VertexArray::new(
            gl,
            vec![BufferLayout::new(
                vec![
                    BufferElement::new(ShaderDataType::Float2, "positions"),
                    BufferElement::new(ShaderDataType::Float2, "dimensions"),
                    BufferElement::new(ShaderDataType::Float3, "colours"),
                    BufferElement::new(ShaderDataType::Float1, "opacity"),
                    BufferElement::new(ShaderDataType::Float3, "border_colours"),
                    BufferElement::new(ShaderDataType::Float4, "border_radii"),
                    BufferElement::new(ShaderDataType::Float1, "border_weights"),
                ],
                QUAD_BUFFER_SIZE,
                1,
                BufferUsage::DynamicDraw,
            )],
        );

        // TODO: for now this is all we have, so left it bound
        vertex_array.bind(gl);
        vertex_array.vertex_buffers[0].bind(gl);
        program.bind(gl);

        // push all index data at once since it should never need to change, hence usage of StaticDraw
        // let index_data: Vec<u32> = (0..MAX_VERTICES as u32)
        //     .step_by(4)
        //     .flat_map(|i| [i, i + 1, i + 2, i + 2, i + 3, i])
        //     .collect();
        // vertex_array.index_buffer.push_data(&gl, 0, &index_data);

        Self {
            program,
            vertex_array,
            transform_loc,
            buffer: Vec::new(),
        }
    }

    pub fn update_transform(&self, gl: &glow::Context, transform: Mat4f) {
        unsafe {
            gl.uniform_matrix_4_f32_slice(Some(&self.transform_loc), false, transform.as_ref());
        }
    }

    pub fn buffer(&mut self, quad: mocha_core::Quad) {
        self.buffer.push(quad);
    }

    pub fn flush(&mut self, gl: &glow::Context) {
        unsafe {
            self.vertex_array.vertex_buffers[0].push_data(gl, 0, &self.buffer);
            gl.draw_arrays_instanced(glow::TRIANGLE_STRIP, 0, 4, self.buffer.len() as i32);
        }

        self.buffer.clear();
    }
}
