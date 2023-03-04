pub const LIGHT: StyleSheet = StyleSheet {
    rotate: 0.0,
    opacity: 1.0,
    background_colour: Colour::white(),
    font_size: 16.0,
    font_weight: 600.0,
    font_colour: Colour::black(),
    border_weight: 0.0,
    border_colour: Colour::black(),
    border_radius: BorderRadius::none(),
};

pub const DARK: StyleSheet = StyleSheet {
    rotate: 0.0,
    opacity: 1.0,
    background_colour: Colour::black(),
    font_size: 16.0,
    font_weight: 600.0,
    font_colour: Colour::white(),
    border_weight: 0.0,
    border_colour: Colour::white(),
    border_radius: BorderRadius::none(),
};

pub struct StyleSheet {
    pub rotate: f32,
    pub opacity: f32,

    pub background_colour: Colour,
    pub font_size: f32,
    pub font_weight: f32,
    pub font_colour: Colour,
    pub border_weight: f32,
    pub border_colour: Colour,
    pub border_radius: BorderRadius,
}

pub struct ComputedStyle {
    pub rotate: f32,
    pub opacity: f32,

    pub background_colour: Colour,
    pub font_size: f32,
    pub font_weight: f32,
    pub font_colour: Colour,
    pub border_weight: f32,
    pub border_colour: Colour,
    pub border_radius: BorderRadius,
}

pub enum Theme {
    Light(StyleSheet),
    Dark(StyleSheet),
    Custom(StyleSheet),
}

#[derive(Clone, Copy)]
pub struct BorderRadius {
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
    pub top_left: f32,
}

impl BorderRadius {
    pub const fn none() -> Self {
        Self {
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: 0.0,
            top_left: 0.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub const fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub const fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}
