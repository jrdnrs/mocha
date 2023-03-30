use std::num::NonZeroU32;

use glutin::surface::GlSurface;
use mocha_core::{Renderer, component::{Layer, Component}};
use winit::event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent};

use crate::{graphics::renderer::GlRenderer, window::window::GlWindow};

pub fn execute() {
    let mut app = mocha_core::Application::<GlRenderer, GlWindow>::new();
    let event_loop = app.window.event_loop.take().unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::LoopDestroyed => {}
            Event::Resumed => {}
            Event::Suspended => {}
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    // self.input.mouse.delta_x += delta.0;
                    // self.input.mouse.delta_y += delta.1;
                    // self.input.mouse.moved = true;
                }
                _ => (),
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    // self.input.mouse.handle_input(button, state);
                }

                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            VirtualKeyCode::F1 => {
                            }
                            _ => {}
                        }
                    }
                }

                WindowEvent::CursorEntered { .. } => {
                    // self.input.mouse.on_window = true;
                }

                WindowEvent::CursorLeft { .. } => {
                    // self.input.mouse.on_window = false;
                }

                WindowEvent::CursorMoved { position, .. } => {
                    // self.input.mouse.pos_x = position.x;
                    // self.input.mouse.pos_y = position.y;
                }

                WindowEvent::Resized(ref size) => {
                    if size.width != 0 && size.height != 0 {
                        // Some platforms like EGL require resizing GL surface to update the size
                        // Notable platforms here are Wayland and macOS, other don't require it
                        // and the function is no-op, but it's wise to resize it for portability
                        // reasons.
                        app.window.surface.resize(
                            &app.window.gl_context,
                            NonZeroU32::new(size.width).unwrap(),
                            NonZeroU32::new(size.height).unwrap(),
                        );
                        app.renderer.update_viewport(size.width, size.height);
                    }
                }
                WindowEvent::Moved(_) => {
                    // self.context.being_moved = true;
                }
                WindowEvent::CloseRequested => control_flow.set_exit(),
                _ => {}
            },
            Event::MainEventsCleared => {
                app.window.window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                app.draw();

                app.window
                    .surface
                    .swap_buffers(&app.window.gl_context)
                    .unwrap();

                // let now = time::Instant::now();
                // self.context.last_frame_delta = now - self.context.last_frame_time;
                // self.context.last_frame_time = time::Instant::now();
            }
            _ => (),
        }
    });
}
