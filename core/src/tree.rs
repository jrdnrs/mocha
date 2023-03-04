use std::ptr::NonNull;

pub struct Tree<T> {
    pub storage: Vec<T>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self { storage: Vec::new() }
    }
}

pub struct Node<T> {
    pub id: usize,
    pub tree: NonNull<Tree<T>>,
    pub root: NonNull<Node<T>>,
    pub data: NonNull<T>,
    pub parent: Option<NonNull<Node<T>>>,
    pub children: Option<Vec<NonNull<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn parent_mut(&self) -> Option<&mut Node<T>> {
        self.parent.map(|p| unsafe { p.as_ptr().as_mut().unwrap()} )
    }

    pub fn push_child(&mut self, data: T) {
        let tree = unsafe {self.tree.as_ptr().as_mut().unwrap()};

        tree.storage.push(data)

    }
}


mod test {
    use super::*;

    #[test]
    fn main() {

    }
}