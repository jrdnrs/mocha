use crate::Renderer;

pub trait Window {
    fn new(config: WindowConfig) -> Self;
    fn init_renderer<R: Renderer>(&self) -> R;
    fn set_title(&mut self, title: &str);
    fn get_title(&self) -> &str;
    fn set_size(&mut self, width: u32, height: u32);
    fn get_size(&self) -> (u32, u32);
    fn set_resizable(&mut self, resizable: bool);
    fn get_resizable(&self) -> bool;
    fn set_maximised(&mut self, maximised: bool);
    fn get_maximised(&self) -> bool;
    fn set_fullscreen(&mut self, fullscreen: bool);
    fn get_fullscreen(&self) -> bool;
}

pub struct WindowConfig {
    pub title: &'static str,
    pub size: (u32, u32),
    pub resizable: bool,
    pub maximised: bool,
    pub fullscreen: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Mocha App",
            size: (640, 480),
            resizable: true,
            maximised: false,
            fullscreen: false,
        }
    }
}
