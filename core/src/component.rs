use std::marker::PhantomData;

use crate::{
    event::Event,
    layout::{ComputedLayout, Display, Layout, Padding, Point, Position, Size},
    style::{ComputedStyle, StyleSheet, LIGHT},
};
use trees::{Node, Tree};

pub struct Environment;

pub struct Layer {
    pub component: Tree<Component>,
    pub layout: Tree<ComputedLayout>,
    pub style: Tree<ComputedStyle>,
}

pub struct Components {
    pub layers: Vec<Layer>,
}

impl Components {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn build_layers(&mut self, mut comp_tree: Tree<Component>) {
        let mut split_comp_trees = Vec::new();
        Self::split_trees(&mut comp_tree.root_mut(), &mut split_comp_trees);
        split_comp_trees.push(comp_tree);
        split_comp_trees.reverse();
        for comp_tree in split_comp_trees {
            let layout_tree = Self::build_layout_tree(&comp_tree);
            let style_tree = Self::build_style_tree(&comp_tree);

            self.layers.push(Layer {
                component: comp_tree,
                layout: layout_tree,
                style: style_tree,
            })
        }
    }

    fn split_trees(node: &mut Node<Component>, split: &mut Vec<Tree<Component>>) {
        for mut child in node.iter_mut() {
            if !child.has_no_child() {
                Self::split_trees(&mut child, split);
            }
            match child.data().layout.position {
                Position::Relative(_) | Position::Absolute(_, _) => {
                    split.push(child.detach());
                }
                _ => {}
            }
        }
    }

    fn build_style_tree(comp_tree: &Tree<Component>) -> Tree<ComputedStyle> {
        let root_style = Self::compute_style(&comp_tree.root().data().style);
        let mut style_tree = Tree::new(root_style);
        Self::recurse_style_tree(&comp_tree, &mut style_tree.root_mut());

        style_tree
    }

    fn recurse_style_tree(root: &Node<Component>, root_style: &mut Node<ComputedStyle>) {
        if root.has_no_child() {
            return;
        }

        for child in root.iter() {
            let style = &child.data().style;
            let child_style = Self::compute_style(style);
            root_style.push_back(Tree::new(child_style));

            Self::recurse_style_tree(child, &mut root_style.back_mut().unwrap());
        }
    }

    fn build_layout_tree(comp_tree: &Tree<Component>) -> Tree<ComputedLayout> {
        let root_layout = ComputedLayout {
            width: 1600.0,
            height: 900.0,
            position: Point { x: 0.0, y: 0.0 },
        };
        let mut layout_tree = Tree::new(root_layout);
        Self::recurse_layout_tree(&comp_tree, &mut layout_tree.root_mut());

        layout_tree
    }

    fn recurse_layout_tree(root: &Node<Component>, root_layout: &mut Node<ComputedLayout>) {
        if root.has_no_child() {
            return;
        }

        for child in root.iter() {
            let child_layout = Self::compute_layout(
                &child.data().layout,
                &root.data().layout,
                &root_layout.data(),
                root.degree(),
            );
            root_layout.push_back(Tree::new(child_layout));

            Self::recurse_layout_tree(child, &mut root_layout.back_mut().unwrap());
        }
    }

    fn compute_style(style_conf: &StyleSheet) -> ComputedStyle {
        ComputedStyle {
            rotate: style_conf.rotate,
            opacity: style_conf.opacity,
            background_colour: style_conf.background_colour,
            font_size: style_conf.font_size,
            font_weight: style_conf.font_weight,
            font_colour: style_conf.font_colour,
            border_weight: style_conf.border_weight,
            border_colour: style_conf.border_colour,
            border_radius: style_conf.border_radius,
        }
    }

    fn compute_layout(
        layout_conf: &Layout,
        parent_layout_conf: &Layout,
        parent_layout: &ComputedLayout,
        parent_degree: usize,
    ) -> ComputedLayout {
        let parent_degree = parent_degree as f32;
        let width = match layout_conf.width {
            Size::Constant(value) => value,
            Size::Fill => {
                parent_layout.width
                    - (parent_layout_conf.padding.left + parent_layout_conf.padding.right)
                        / parent_degree
            }
        };
        let height = match layout_conf.height {
            Size::Constant(value) => value,
            Size::Fill => {
                parent_layout.height
                    - (parent_layout_conf.padding.top + parent_layout_conf.padding.bottom)
                        / parent_degree
            }
        };
        let position = Point {
            x: parent_layout.position.x + parent_layout_conf.padding.left,
            y: parent_layout.position.y + parent_layout_conf.padding.top,
        };

        ComputedLayout {
            height,
            width,
            position,
        }
    }
}

pub trait Message {
    type Data;
}

pub trait Handler<M: Message> {}
impl<F, M: Message> Handler<M> for F where F: FnMut(M::Data) {}

pub trait MessageHandlerTrait {
    fn handle(&self);
}

pub struct MessageHandler<M: Message, H: Handler<M>> {
    handler: H,
    param: PhantomData<M>,
}

impl<M: Message, H: Handler<M>> MessageHandlerTrait for MessageHandler<M, H> {
    fn handle(&self) {
        // extract params and use handler on self
    }
}

pub fn into_MessageHandlerData<M: Message, H: Handler<M>>(handler: H) -> impl MessageHandlerTrait {
    MessageHandler {
        handler: handler,
        param: PhantomData,
    }
}


pub struct Component {
    pub style: StyleSheet,
    pub layout: Layout,
    // pub connections: Vec<Box<dyn Msg>>
}

impl Component {
    pub fn new() -> Self {
        Self {
            style: LIGHT,
            layout: Layout {
                display: Display::Row,
                position: Position::Static,
                height: Size::Constant(200.0),
                width: Size::Constant(300.0),
                padding: Padding::none(),
            },
        }
    }

    pub fn event(&mut self, event: Event) {}

    pub fn connect<M: Message>(&mut self, slot: &mut Component, handler: impl Handler<M>) {}

    pub fn emit(&self) {}

    pub fn repaint(&self, env: &mut Environment) {}

    pub fn relayout(&self, env: &mut Environment) {}
}

mod test {
    use crate::layout::{Anchor, Offset};

    use super::*;

    #[test]
    fn simple() {
        let root_component = Component::new();
        let mut comp1 = Component::new();
        comp1.layout.height = Size::Fill;
        let mut comp2 = Component::new();
        comp2.layout.height = Size::Fill;

        let mut layers = Components::new();
        let all_components = Tree::<Component>::from_tuple((root_component, (comp1, comp2)));
        layers.build_layers(all_components);

        assert_eq!(layers.layers.len(), 1);
        assert_eq!(layers.layers[0].component.degree(), 1);
        assert_eq!(layers.layers[0].component.node_count(), 3);
        assert_eq!(layers.layers[0].layout.degree(), 1);
        assert_eq!(layers.layers[0].layout.node_count(), 3);
        assert_eq!(layers.layers[0].style.degree(), 1);
        assert_eq!(layers.layers[0].style.node_count(), 3);
    }

    #[test]
    fn multiple_layers() {
        let root_component = Component::new();
        let mut comp1 = Component::new();
        comp1.layout.height = Size::Fill;
        let mut comp2 = Component::new();
        comp2.layout.position = Position::Absolute(Offset { x: 10.0, y: 10.0 }, Anchor::Top);
        let mut comp3 = Component::new();
        comp3.layout.height = Size::Fill;

        let mut layers = Components::new();
        let all_components =
            Tree::<Component>::from_tuple((root_component, (comp1, comp2, (comp3))));
        layers.build_layers(all_components);

        assert_eq!(layers.layers.len(), 2);
    }
}
