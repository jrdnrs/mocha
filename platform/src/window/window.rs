use std::{ffi::CString, num::NonZeroU32};

use glutin::{
    config::{Config, ConfigTemplateBuilder},
    context::{ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::*,
    surface::{Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::Window,
    window::{Fullscreen, WindowBuilder},
};

const SAMPLES: u8 = 0;

pub struct GlWindow {
    // the surface must be dropped before the window.
    pub(crate) surface: Surface<WindowSurface>,
    pub(crate) window: Window,
    pub(crate) event_loop: Option<EventLoop<()>>,

    pub(crate) gl_config: Config,
    pub(crate) gl_context: PossiblyCurrentContext,

    pub(crate) title: String,
    pub(crate) resizable: bool,
    pub(crate) maximised: bool,
    pub(crate) fullscreen: Option<Fullscreen>,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl GlWindow {
    pub fn new(window_builder: WindowBuilder, display_config: ConfigTemplateBuilder) -> Self {
        let event_loop = EventLoop::new();

        let (window, gl_config) = Self::build_window(window_builder, display_config, &event_loop);

        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.raw_window_handle();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = Self::create_gl_context(&window, &gl_config).treat_as_possibly_current();

        Self {
            surface,
            window,
            event_loop: Some(event_loop),
            gl_config,
            gl_context,
            title: String::new(),
            resizable: true,
            maximised: false,
            fullscreen: None,
            width,
            height,
        }
    }

    pub fn make_context_current(&self) {
        self.gl_context
            .make_current(&self.surface)
            .expect("failed to make context current");
    }

    pub fn make_context_not_current(&self) {
        // self.gl_context
        //     .make_not_current()
        //     .expect("failed to make context not current");
    }

    fn build_window(
        window_builder: WindowBuilder,
        display_config: ConfigTemplateBuilder,
        event_loop: &EventLoop<()>,
    ) -> (Window, Config) {
        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(&event_loop, display_config, |configs| {
                // iter configs to find one that best supports our template config
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() == SAMPLES && accum.num_samples() != SAMPLES {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        (window.expect("Failed to create window"), gl_config)
    }

    fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
        let raw_window_handle = window.raw_window_handle();

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(None))
            .build(Some(raw_window_handle));

        // try Gles as backup
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let gl_display = gl_config.display();

        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        };

        not_current_gl_context
    }
}

impl mocha_core::Window for GlWindow {
    fn new(config: mocha_core::window::WindowConfig) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title(config.title)
            .with_inner_size(LogicalSize::new(config.size.0, config.size.1))
            .with_resizable(config.resizable)
            .with_maximized(config.maximised);
        // .with_fullscreen(fullscreen);

        // template of desired configs for display context
        let display_config = ConfigTemplateBuilder::new();

        let mut gl_window = GlWindow::new(window_builder, display_config);
        gl_window.title = config.title.to_string();
        gl_window.width = config.size.0;
        gl_window.height = config.size.1;
        gl_window.resizable = config.resizable;
        gl_window.maximised = config.maximised;

        gl_window.make_context_current();
        if let Err(res) = gl_window.surface.set_swap_interval(
            &gl_window.gl_context,
            SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
        ) {
            println!("Error setting vsync: {:?}", res);
        }

        println!("Display API: {}", gl_window.gl_config.display().version_string());
        println!("Samples: {}", gl_window.gl_config.num_samples());

        gl_window
    }

    fn init_renderer<R: mocha_core::Renderer>(&self) -> R {
        R::from_loader_function(|s| {
            self.gl_config
                .display()
                .get_proc_address(&CString::new(s).unwrap())
        })
    }

    fn set_title(&mut self, title: &str) {
        if self.title == title {
            return;
        };
        self.title = title.to_owned();
        self.window.set_title(&self.title);
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn set_size(&mut self, width: u32, height: u32) {
        if self.width == width && self.height == height {
            return;
        }

        self.width = width;
        self.height = height;
        self.surface.resize(
            &self.gl_context,
            NonZeroU32::new(width).expect("width must be greater than zero"),
            NonZeroU32::new(height).expect("height must be greater than zero"),
        )
    }

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn set_resizable(&mut self, resizable: bool) {
        if self.resizable == resizable {
            return;
        }

        self.resizable = resizable;
        self.window.set_resizable(resizable);
    }

    fn get_resizable(&self) -> bool {
        self.resizable
    }

    fn set_maximised(&mut self, maximised: bool) {
        if self.maximised == maximised {
            return;
        }

        self.maximised = maximised;
        self.window.set_maximized(maximised);
    }

    fn get_maximised(&self) -> bool {
        self.maximised
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        if fullscreen {
            // TODO: implement
            // self.window.set_fullscreen(Some(Fullscreen::Exclusive(VideoMode::)));
        } else {
            self.window.set_fullscreen(None);
        }
    }

    fn get_fullscreen(&self) -> bool {
        self.fullscreen.is_some()
    }
}
