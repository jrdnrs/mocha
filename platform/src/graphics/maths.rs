/// column-major
pub struct Mat4f(pub [f32; 16]);

impl Mat4f {
    #![cfg_attr(rustfmt, rustfmt_skip)]
    
    pub fn identity() -> Self {
        Mat4f([
            1.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn orthographic_2d(width: f32, height: f32) -> Self {
        // scale
        let s_x = 2.0 / width;
        let s_y = 2.0 / height;

        // offset to top-left
        let o_x = -1.0;
        let o_y =  1.0;

        Mat4f([
            s_x, 0.0, 0.0, 0.0, 
            0.0, s_y, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            o_x, o_y, 0.0, 1.0,
        ])
    }
}

impl AsRef<[f32]> for Mat4f {
    fn as_ref(&self) -> &[f32] {
        &self.0
    }
}
