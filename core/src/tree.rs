pub struct Node<T> {
    pub data: T,
    pub children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            children: Vec::new(),
        }
    }

    pub fn push_child(&mut self, node: Node<T>) {
        self.children.push(node);
    }

    pub fn has_no_children(&self) -> bool {
        self.children.is_empty()
    }
}

mod test {
    use super::*;

    #[test]
    fn main() {
        let mut root = Node::new(0);

        let one = Node::new(1);
        let mut two = Node::new(2);

        let three = Node::new(3);
        let four = Node::new(4);
        let five = Node::new(5);

        two.push_child(three);
        two.push_child(four);
        two.push_child(five);

        root.push_child(one);
        root.push_child(two);

        print_data(&mut root);
    }

    fn print_data(node: &mut Node<i32>) {
        if node.has_no_children() {
            return;
        }

        for child in &mut node.children {
            println!("{}", child.data);
            print_data(child);
        }
    }
}
