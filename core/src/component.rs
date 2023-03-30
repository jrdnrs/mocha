use crate::{
    layout::{ComputedLayout, Dimensions, Layout, Orientation, Padding, Point, Position, Size},
    style::{StyleSheet, LIGHT},
    tree::Node,
};

pub struct Environment;

pub struct Layer {
    pub component_tree: Node<Component>,
}

pub struct Components {
    pub layers: Vec<Layer>,
}

impl Components {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn compute_layout(&mut self, screen: Dimensions) {
        // for now just rebuild the entire tree based on current state of components
        for layer in &mut self.layers {
            // establish root for computed layout
            let root_component = &mut layer.component_tree.data;
            root_component.computed_layout.position = match &root_component.layout.position {
                Position::Static => Point { x: 0.0, y: 0.0 },
                Position::Relative(offset) => Point {
                    x: offset.x,
                    y: offset.y,
                },
                Position::Absolute(offset, anchor) => {
                    let anchor_point = match anchor {
                        crate::layout::Anchor::Top => Point {
                            x: screen.width / 2.0,
                            y: 0.0,
                        },
                        crate::layout::Anchor::TopRight => Point {
                            x: screen.width,
                            y: 0.0,
                        },
                        crate::layout::Anchor::Right => Point {
                            x: screen.width,
                            y: screen.height / 2.0,
                        },
                        crate::layout::Anchor::BottomRight => Point {
                            x: screen.width,
                            y: screen.height,
                        },
                        crate::layout::Anchor::Bottom => Point {
                            x: screen.width / 2.0,
                            y: screen.height,
                        },
                        crate::layout::Anchor::BottomLeft => Point {
                            x: 0.0,
                            y: screen.height,
                        },
                        crate::layout::Anchor::Left => Point {
                            x: 0.0,
                            y: screen.height / 2.0,
                        },
                        crate::layout::Anchor::TopLeft => Point { x: 0.0, y: 0.0 },
                    };
                    Point {
                        x: anchor_point.x + offset.x,
                        y: anchor_point.y + offset.y,
                    }
                }
            };

            root_component.computed_layout.width = match root_component.layout.width {
                Size::Constant(size) => size,
                Size::Fill => screen.width,
            };

            root_component.computed_layout.height = match root_component.layout.height {
                Size::Constant(size) => size,
                Size::Fill => screen.height,
            };

            Self::resolve_child_dimensions(&mut layer.component_tree);
            Self::resolve_child_positions(&mut layer.component_tree);
            Self::compute_child_layouts(&mut layer.component_tree);
        }
    }

    fn compute_child_layouts(comp_tree: &mut Node<Component>) {
        for node in &mut comp_tree.children {
            Self::resolve_child_dimensions(node);
            Self::resolve_child_positions(node);

            Self::compute_child_layouts(node)
        }
    }

    /// Resolves the screen position of the children of the given node
    /// This process is contigent on the width/height and position of the given node
    fn resolve_child_positions(comp_tree: &mut Node<Component>) {
        if comp_tree.has_no_children() {
            return;
        }

        let parent_orient = &comp_tree.data.layout.orientation;
        let parent_pos = &comp_tree.data.computed_layout.position;
        let parent_w = comp_tree.data.computed_layout.width;
        let parent_h = comp_tree.data.computed_layout.height;
        let mut flow_offset_x = 0.0;
        let mut flow_offset_y = 0.0;

        for child in &mut comp_tree.children {
            child.data.computed_layout.position = match &child.data.layout.position {
                Position::Static => match parent_orient {
                    Orientation::Row => {
                        let p = Point {
                            x: parent_pos.x + flow_offset_x,
                            y: parent_pos.y,
                        };
                        flow_offset_x += child.data.computed_layout.width;
                        p
                    }
                    Orientation::Column => {
                        let p = Point {
                            x: parent_pos.x,
                            y: parent_pos.y + flow_offset_y,
                        };
                        flow_offset_y += child.data.computed_layout.height;
                        p
                    }
                },
                Position::Relative(offset) => match parent_orient {
                    Orientation::Row => {
                        let p = Point {
                            x: parent_pos.x + offset.x + flow_offset_x,
                            y: parent_pos.y + offset.y,
                        };
                        flow_offset_x += child.data.computed_layout.width;
                        p
                    }
                    Orientation::Column => {
                        let p = Point {
                            x: parent_pos.x + offset.x,
                            y: parent_pos.y + offset.y + flow_offset_y,
                        };
                        flow_offset_y += child.data.computed_layout.height;
                        p
                    }
                },
                Position::Absolute(offset, anchor) => {
                    let anchor_point = match anchor {
                        crate::layout::Anchor::Top => Point {
                            x: parent_pos.x + (parent_w - child.data.computed_layout.width) / 2.0,
                            y: parent_pos.y,
                        },
                        crate::layout::Anchor::TopRight => Point {
                            x: parent_pos.x + parent_w - child.data.computed_layout.width,
                            y: parent_pos.y,
                        },
                        crate::layout::Anchor::Right => Point {
                            x: parent_pos.x + parent_w - child.data.computed_layout.width,
                            y: parent_pos.y + (parent_h - child.data.computed_layout.height) / 2.0,
                        },
                        crate::layout::Anchor::BottomRight => Point {
                            x: parent_pos.x + parent_w - child.data.computed_layout.width,
                            y: parent_pos.y + parent_h - child.data.computed_layout.height,
                        },
                        crate::layout::Anchor::Bottom => Point {
                            x: parent_pos.x + (parent_w - child.data.computed_layout.width) / 2.0,
                            y: parent_pos.y + parent_h - child.data.computed_layout.height,
                        },
                        crate::layout::Anchor::BottomLeft => Point {
                            x: parent_pos.x,
                            y: parent_pos.y + parent_h - child.data.computed_layout.height,
                        },
                        crate::layout::Anchor::Left => Point {
                            x: parent_pos.x,
                            y: parent_pos.y + (parent_h - child.data.computed_layout.height) / 2.0,
                        },
                        crate::layout::Anchor::TopLeft => parent_pos.clone(),
                    };
                    Point {
                        x: anchor_point.x + offset.x,
                        y: anchor_point.y + offset.y,
                    }
                }
            };
        }
    }

    /// Resolves the width and height of the children of the given node.
    /// This process is contigent on the width/height of the given node
    fn resolve_child_dimensions(comp_tree: &mut Node<Component>) {
        if comp_tree.has_no_children() {
            return;
        }

        let parent_width = comp_tree.data.computed_layout.width;
        let parent_height = comp_tree.data.computed_layout.height;
        let mut children_total_const_w = 0.0;
        let mut children_total_const_h = 0.0;
        let mut children_fill_w_count = 0.0;
        let mut children_fill_h_count = 0.0;

        for child in &comp_tree.children {
            if let Position::Absolute(_, _) = child.data.layout.position {
                continue;
            }

            match &child.data.layout.width {
                Size::Constant(size) => children_total_const_w += size,
                Size::Fill => children_fill_w_count += 1.0,
            }

            match &child.data.layout.height {
                Size::Constant(size) => children_total_const_h += size,
                Size::Fill => children_fill_h_count += 1.0,
            }
        }

        let (children_auto_width, children_auto_height) = match &comp_tree.data.layout.orientation {
            Orientation::Row => (
                (parent_width - children_total_const_w) / children_fill_w_count,
                parent_height,
            ),
            Orientation::Column => (
                parent_width,
                (parent_height - children_total_const_h) / children_fill_h_count,
            ),
        };

        for child in &mut comp_tree.children {
            match &child.data.layout.width {
                Size::Constant(size) => child.data.computed_layout.width = *size,
                Size::Fill => child.data.computed_layout.width = children_auto_width,
            }

            match &child.data.layout.height {
                Size::Constant(size) => child.data.computed_layout.height = *size,
                Size::Fill => child.data.computed_layout.height = children_auto_height,
            }
        }
    }
}

pub struct Component {
    pub style: StyleSheet,
    pub layout: Layout,
    pub computed_layout: ComputedLayout,
    // pub connections: Vec<Box<dyn Msg>>
}

impl Component {
    pub fn new() -> Self {
        Self {
            style: LIGHT,
            layout: Layout {
                orientation: Orientation::Row,
                position: Position::Static,
                height: Size::Constant(200.0),
                width: Size::Constant(300.0),
                padding: Padding::none(),
            },
            computed_layout: ComputedLayout::default(),
        }
    }

    pub fn repaint(&self, env: &mut Environment) {}

    pub fn relayout(&self, env: &mut Environment) {}
}
