use crate::graphics::backend::QuadPipeline;
use glow::HasContext;

use super::maths::Mat4f;

pub struct GlRenderer {
    gl: glow::Context,

    quad_pipeline: QuadPipeline,
}

impl mocha_core::Renderer for GlRenderer {
    fn new() -> Self {
        unimplemented!();
    }

    fn from_loader_function<F>(loader_function: F) -> Self
    where
        F: FnMut(&str) -> *const std::os::raw::c_void,
    {
        let gl = unsafe { glow::Context::from_loader_function(loader_function) };

        unsafe {
            println!("GL Version: {}", gl.get_parameter_string(glow::VERSION));

            let gl_version = gl.get_parameter_i32(glow::MAJOR_VERSION) * 10
                + gl.get_parameter_i32(glow::MINOR_VERSION);

            // Requires OpenGL 4.3 (No ES)
            if gl_version >= 43 {
                gl.debug_message_callback(|src, typ, id, sev, msg: &str| {
                    let source = error_source(src);
                    let severity = error_severity(sev);
                    let type_ = error_type(typ);
                    println!(
                        "{} :: {} :: {} :: {} :: {}",
                        id, severity, type_, source, msg
                    );
                });
            }

            gl.disable(glow::DEPTH_TEST);
            gl.disable(glow::SCISSOR_TEST);
            gl.disable(glow::STENCIL_TEST);
            gl.disable(glow::CULL_FACE);
            gl.depth_mask(false);
            gl.front_face(glow::CW);
            gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        }

        let quad_pipeline = QuadPipeline::new(&gl);
        quad_pipeline.update_transform(&gl, Mat4f::identity());

        Self { gl, quad_pipeline }
    }

    fn update_viewport(&self, width: u32, height: u32) {
        self.quad_pipeline.update_transform(
            &self.gl,
            Mat4f::orthographic_2d(width as f32, height as f32),
        );

        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    fn draw(&mut self, quad: mocha_core::Quad) {
        self.quad_pipeline.buffer(quad);
    }

    fn present(&mut self) {
        unsafe {
            self.gl.clear_color(0.5, 0.5, 0.0, 0.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }

        self.quad_pipeline.flush(&self.gl);
    }
}

fn error_source(source: u32) -> &'static str {
    match source {
        glow::DEBUG_SOURCE_API => "Calls to the OpenGL API",
        glow::DEBUG_SOURCE_WINDOW_SYSTEM => "Calls to a window-system API",
        glow::DEBUG_SOURCE_SHADER_COMPILER => "A compiler for a shading language",
        glow::DEBUG_SOURCE_THIRD_PARTY => "An application associated with OpenGL",
        glow::DEBUG_SOURCE_APPLICATION => "Generated by the user of this application",
        _ => "",
    }
}

fn error_severity(severity: u32) -> &'static str {
    match severity {
        glow::DEBUG_SEVERITY_HIGH => "High",
        glow::DEBUG_SEVERITY_MEDIUM => "Medium",
        glow::DEBUG_SEVERITY_LOW => "Low",
        glow::DEBUG_SEVERITY_NOTIFICATION => "Notification",
        _ => "",
    }
}

fn error_type(type_: u32) -> &'static str {
    match type_ {
        glow::DEBUG_TYPE_ERROR => "An error, typically from the API",
        glow::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Some behavior marked deprecated has been used",
        glow::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Something has invoked undefined behavior",
        glow::DEBUG_TYPE_PORTABILITY => "Some functionality the user relies upon is not portable",
        glow::DEBUG_TYPE_PERFORMANCE => "	Code has triggered possible performance issues",
        glow::DEBUG_TYPE_MARKER => "Command stream annotation",
        glow::DEBUG_TYPE_PUSH_GROUP => "Group pushing",
        glow::DEBUG_TYPE_POP_GROUP => "Group popping",
        _ => "",
    }
}
