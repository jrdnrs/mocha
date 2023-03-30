use std::collections::VecDeque;

use crate::{
    component::{Component, Components, Layer},
    event::Event,
    layout::{ComputedLayout, Dimensions, Offset, Orientation, Position, Size},
    style::{BorderRadius, Colour, StyleSheet},
    tree::Node,
    window::{Window, WindowConfig},
    Quad, Renderer,
};

pub struct Application<R: Renderer, W: Window> {
    pub renderer: R,
    pub window: W,
    pub components: Components,
}

impl<R: Renderer, W: Window> Application<R, W> {
    pub fn new() -> Self {
        let window = W::new(WindowConfig::default());
        let renderer = window.init_renderer();
        let mut components = Components::new();

        let mut root_comp = Node::new(Component::new());
        root_comp.data.layout.width = Size::Fill;
        root_comp.data.layout.height = Size::Fill;
        root_comp.data.style.background_colour = Colour::new(1.0, 0.5, 0.0);

        let mut child_1 = Node::new(Component::new());
        child_1.data.layout.position = Position::Absolute(
            Offset::new(0.0, 100.0),
            crate::layout::Anchor::Top,
        );
        child_1.data.style.background_colour = Colour::new(0.0, 0.5, 0.0);
        child_1.data.layout.height = Size::Constant(50.0);
        child_1.data.layout.width = Size::Constant(50.0);
        child_1.data.style.border_radius = BorderRadius::new(5.0, 20.0, 5.0, 20.0);


        let mut child_2 = Node::new(Component::new());
        child_2.data.style.background_colour = Colour::new(1.0, 0.5, 0.5);
        child_2.data.layout.height = Size::Constant(50.0);
        child_2.data.layout.width = Size::Fill;

        let mut child_3 = Node::new(Component::new());
        child_3.data.style.background_colour = Colour::new(0.5, 0.0, 0.0);
        child_3.data.layout.height = Size::Fill;
        child_3.data.layout.width = Size::Constant(50.0);
 

        root_comp.push_child(child_1);
        root_comp.push_child(child_2);
        root_comp.push_child(child_3);
        components.layers.push(Layer {
            component_tree: root_comp,
        });

        Self {
            renderer,
            window,
            components,
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        // if pointer event, run hit test?

        // else, push into tree (components)
    }

    pub fn draw(&mut self) {
        let window_size = self.window.get_size();

        self.components.compute_layout(Dimensions {
            width: window_size.0 as f32,
            height: window_size.1 as f32,
        });

        let layer = &self.components.layers[0];

        let mut to_visit = VecDeque::new();
        to_visit.push_back(&layer.component_tree);

        while !to_visit.is_empty() {
            let node = to_visit.pop_front().unwrap();
            self.renderer
                .draw(create_quad(&node.data.style, &node.data.computed_layout));
            for child in &node.children {
                to_visit.push_back(child)
            }
        }

        // for (style, layout) in layer.style.iter().zip(layer.layout.iter()) {
        //     let layout_data = layout.data();
        //     let style_data = style.data();

        //     self.renderer.draw(Quad {
        //         position: [layout_data.position.x, layout_data.position.y],
        //         dimensions: [layout_data.width, layout_data.height],
        //         colour: [
        //             style_data.background_colour.r,
        //             style_data.background_colour.g,
        //             style_data.background_colour.b,
        //         ],
        //         opacity: style_data.opacity,
        //         border_colour: [
        //             style_data.border_colour.r,
        //             style_data.border_colour.g,
        //             style_data.border_colour.b,
        //         ],
        //         border_radius: [
        //             style_data.border_radius.top_right,
        //             style_data.border_radius.bottom_right,
        //             style_data.border_radius.bottom_left,
        //             style_data.border_radius.top_left,
        //         ],
        //         border_weight: style_data.border_weight,
        //     })
        // }

        // self.renderer.draw(Quad {
        //     position: [10.0, 10.0],
        //     dimensions: [50.0, 50.0],
        //     colour: [1.0, 0.5, 0.0],
        //     opacity: 1.0,
        //     border_colour: [0.0, 0.0, 1.0],
        //     border_radius: [0.0, 0.0, 0.0, 0.0],
        //     border_weight: 2.0,
        // });

        // self.renderer.draw(Quad {
        //     position: [10.0, 100.0],
        //     dimensions: [200.0, 150.0],
        //     colour: [1.0, 0.5, 0.0],
        //     opacity: 0.5,
        //     border_colour: [0.0, 0.0, 1.0],
        //     border_radius: [5.0, 10.0, 2.0, 50.0],
        //     border_weight: 10.0,
        // });

        self.renderer.present();
    }
}

pub fn create_quad(style: &StyleSheet, layout: &ComputedLayout) -> Quad {
    Quad {
        position: [layout.position.x, layout.position.y],
        dimensions: [layout.width, layout.height],
        colour: [
            style.background_colour.r,
            style.background_colour.g,
            style.background_colour.b,
        ],
        opacity: style.opacity,
        border_colour: [
            style.border_colour.r,
            style.border_colour.g,
            style.border_colour.b,
        ],
        border_radius: [
            style.border_radius.top_right,
            style.border_radius.bottom_right,
            style.border_radius.bottom_left,
            style.border_radius.top_left,
        ],
        border_weight: style.border_weight,
    }
}
