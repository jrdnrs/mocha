
pub struct Layout {
    pub(crate) display: Display,
    pub(crate) position: Position,
    pub(crate) width: Size,
    pub(crate) height: Size,
    pub(crate) padding: Padding,
}

pub struct ComputedLayout {
    pub(crate) position: Point,
    pub(crate) width: f32,
    pub(crate) height: f32,
}


pub enum Size {
    Constant(f32),
    Fill
}

pub enum Display {
    Row,
    Column,
}

pub enum Position {
    Static,
    Relative(Offset),
    Absolute(Offset, Anchor),
}

pub enum Anchor {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

pub struct Offset {
    pub x: f32,
    pub y: f32,
}

impl Offset {
    pub const fn none() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
    pub const fn none() -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
}
