use crate::{
    component::Components,
    event::Event,
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
        let components = Components::new();

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
        // let layer = &self.components.layers[0];

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

        self.renderer.draw(Quad {
            position: [10.0, 10.0],
            dimensions: [50.0, 50.0],
            colour: [1.0, 0.5, 0.0],
            opacity: 1.0,
            border_colour: [0.0, 0.0, 1.0],
            border_radius: [0.0, 0.0, 0.0, 0.0],
            border_weight: 2.0,
        });

        self.renderer.draw(Quad {
            position: [10.0, 100.0],
            dimensions: [200.0, 150.0],
            colour: [1.0, 0.5, 0.0],
            opacity: 0.5,
            border_colour: [0.0, 0.0, 1.0],
            border_radius: [5.0, 10.0, 2.0, 50.0],
            border_weight: 10.0,
        });

        self.renderer.present();
    }
}
