pub struct Layout {
    pub(crate) orientation: Orientation,
    pub(crate) position: Position,
    pub(crate) width: Size,
    pub(crate) height: Size,
    pub(crate) padding: Padding,
}

#[derive(Default)]
pub struct ComputedLayout {
    /// this defines the top-left position of this quad
    pub(crate) position: Point,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

#[derive(Clone)]
pub enum Size {
    Constant(f32),
    Fill,
}

/// Determines how children of this element are positioned
#[derive(Clone)]
pub enum Orientation {
    /// `Row` means that children are placed side-by-side, from left to right, in the order they are defined
    Row,
    /// `Column` means that children are placed vertically, from top to bottom, in the order they are defined
    Column,
}

/// Determines the rules by which this element must adhere to when screenspace position is computed
pub enum Position {
    /// `Static` is the default flow of elements whereby siblings are placed next to each other, and children
    /// are placed within the parent
    Static,
    /// `Relative` uses the default `Static` flow of elements, however, this element may be visually rendered
    /// with some offset
    Relative(Offset),
    /// `Absolute` effectively ignores the default flow and is treated as a new layer.
    /// For this reason, an anchor point must be decided, from which this element can be offset
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
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn none() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Default, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone)]
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
