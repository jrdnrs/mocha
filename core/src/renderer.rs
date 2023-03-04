pub trait Renderer {
    fn new() -> Self;
    fn from_loader_function<F>(loader_function: F) -> Self
    where
        F: FnMut(&str) -> *const std::os::raw::c_void;
    fn update_viewport(&self, width: u32, height: u32);
    fn draw(&mut self, quad: Quad);
    fn present(&mut self);
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Quad {
    pub position: [f32; 2],
    pub dimensions: [f32; 2],
    pub colour: [f32; 3],
    pub opacity: f32,
    pub border_colour: [f32; 3],
    pub border_radius: [f32; 4],
    pub border_weight: f32,
}

unsafe impl bytemuck::Zeroable for Quad {}
unsafe impl bytemuck::Pod for Quad {}
