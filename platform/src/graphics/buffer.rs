use glow::HasContext;

use crate::graphics::shader::ShaderDataType;

#[derive(Clone, Copy)]
pub enum BufferType {
    Vertex = glow::ARRAY_BUFFER as isize,
    Index = glow::ELEMENT_ARRAY_BUFFER as isize,
    DrawIndirectCommand = glow::DRAW_INDIRECT_BUFFER as isize,
    ShaderStorage = glow::SHADER_STORAGE_BUFFER as isize,
    Texture = glow::TEXTURE_BUFFER as isize,
    Uniform = glow::UNIFORM_BUFFER as isize,
    TransformFeedback = glow::TRANSFORM_FEEDBACK_BUFFER as isize,
}

#[derive(Clone, Copy)]
pub enum BufferUsage {
    StaticDraw = glow::STATIC_DRAW as isize,
    StaticRead = glow::STATIC_READ as isize,
    StaticCopy = glow::STATIC_COPY as isize,
    DynamicDraw = glow::DYNAMIC_DRAW as isize,
    DynamicRead = glow::DYNAMIC_READ as isize,
    DynamicCopy = glow::DYNAMIC_COPY as isize,
    StreamDraw = glow::STREAM_DRAW as isize,
    StreamRead = glow::STREAM_READ as isize,
    StreamCopy = glow::STREAM_COPY as isize,
}

/// Describes a single type of data to be used in a gl buffer.\
/// For example, this can be used to desribe data stored in a vertex buffer object that will be presented
/// as input data to a shader.
pub struct BufferElement {
    pub name: &'static str,
    pub data_type: ShaderDataType,
    pub count: u32,
    pub offset: u32,
    pub normalised: bool,
}

impl BufferElement {
    pub fn new(data_type: ShaderDataType, name: &'static str) -> Self {
        let count = data_type.element_count();
        Self {
            name,
            data_type,
            count,
            offset: 0,
            normalised: false,
        }
    }
}

/// The concrete layout of data in a singular gl buffer.\
/// This can be comprised of many [BufferElement] as they will be interleaved into a single buffer.
pub struct BufferLayout {
    pub elements: Vec<BufferElement>,
    pub stride: u32,
    pub buffer_size: u32,
    pub divisor: u32,
    pub usage: BufferUsage,
}

impl BufferLayout {
    pub fn new(
        mut elements: Vec<BufferElement>,
        buffer_size: u32,
        divisor: u32,
        usage: BufferUsage,
    ) -> Self {
        let mut offset = 0;
        for element in elements.iter_mut() {
            element.offset = offset;
            offset += element.data_type.size_bytes();
        }

        Self {
            elements,
            stride: offset,
            buffer_size,
            divisor,
            usage,
        }
    }
}

pub struct BufferStorage {
    pub handle: glow::Buffer,
    pub buffer_type: BufferType,
    pub layout: BufferLayout,
}

impl BufferStorage {
    pub fn new(gl: &glow::Context, buffer_type: BufferType, layout: BufferLayout) -> Self {
        let handle = unsafe { gl.create_buffer().unwrap() };

        unsafe {
            gl.bind_buffer(buffer_type as u32, Some(handle));
            gl.buffer_data_size(
                buffer_type as u32,
                layout.buffer_size as i32,
                layout.usage as u32,
            );
            gl.bind_buffer(buffer_type as u32, None);
        }

        Self {
            handle,
            buffer_type,
            layout,
        }
    }

    /// Expects this [BufferStorage] to already be bound
    pub fn push_data<T: bytemuck::Pod>(
        &mut self,
        gl: &glow::Context,
        offset: i32,
        vertex_data: &[T],
    ) {
        let data: &[u8] = bytemuck::cast_slice(vertex_data);

        unsafe { gl.buffer_sub_data_u8_slice(self.buffer_type as u32, offset, data) }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe { gl.bind_buffer(self.buffer_type as u32, Some(self.handle)) }
    }

    pub fn unbind(gl: &glow::Context, buffer_type: BufferType) {
        unsafe { gl.bind_buffer(buffer_type as u32, None) }
    }
}

pub struct VertexArray {
    pub handle: glow::VertexArray,
    pub vertex_buffers: Vec<BufferStorage>,
}

impl VertexArray {
    pub fn new(gl: &glow::Context, layouts: Vec<BufferLayout>) -> Self {
        let vao = unsafe { gl.create_vertex_array().unwrap() };

        let buffers = layouts
            .into_iter()
            .map(|layout| BufferStorage::new(gl, BufferType::Vertex, layout))
            .collect();

        let vertex_array = Self {
            handle: vao,
            vertex_buffers: buffers,
        };

        vertex_array.attach_buffers(gl);

        vertex_array
    }

    fn attach_buffers(&self, gl: &glow::Context) {
        self.bind(gl);
        // self.attach_index_buffer(gl);
        self.attach_vertex_buffers(gl);
        VertexArray::unbind(gl);
        // BufferStorage::unbind(gl, BufferType::Index);
        BufferStorage::unbind(gl, BufferType::Vertex);
    }

    /// Expects this [VertexArray] to already be bound
    fn attach_vertex_buffers(&self, gl: &glow::Context) {
        let mut attrib_index = 0;

        for buffer in self.vertex_buffers.iter() {
            unsafe {
                // calling bind buffer here for `vertex_attrib_pointer` ops
                buffer.bind(gl);

                for element in buffer.layout.elements.iter() {
                    gl.enable_vertex_attrib_array(attrib_index);
                    
                    let gl_data_type = element.data_type.gl_type();

                    if gl_data_type == glow::FLOAT {
                        gl.vertex_attrib_pointer_f32(
                            attrib_index,
                            element.count as i32,
                            gl_data_type,
                            element.normalised,
                            buffer.layout.stride as i32,
                            element.offset as i32,
                        );
                    } else {
                        gl.vertex_attrib_pointer_i32(
                            attrib_index,
                            element.count as i32,
                            gl_data_type,
                            buffer.layout.stride as i32,
                            element.offset as i32,
                        );
                    }

                    // Requires > OpenGL 3.3 / OpenGL ES 3.0
                    if buffer.layout.divisor > 0 {
                        gl.vertex_attrib_divisor(attrib_index, buffer.layout.divisor);
                    }

                    attrib_index += 1;
                }
            }
        }
    }

    /// Expects this [VertexArray] to already be bound
    // fn attach_index_buffer(&self, gl: &glow::Context) {
    //     self.index_buffer.bind(gl);
    // }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe { gl.bind_vertex_array(Some(self.handle)) }
    }

    pub fn unbind(gl: &glow::Context) {
        unsafe { gl.bind_vertex_array(None) }
    }
}
