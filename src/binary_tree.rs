use std::fmt::Write;


#[derive(Debug)] 
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialEq + std::cmp::Ord> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            // check if the value is less than the current node's value
            std::cmp::Ordering::Less => {
                if let Some(left) = &mut self.left {
                    left.insert(value);
                }
                else {
                    self.left = Some(Box::new(Node::new(value)));
                }
            }
            // check if the value is greater or equal than the current node's value
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
                if let Some(right) = &mut self.right {
                    right.insert(value);
                }
                else {
                    self.right = Some(Box::new(Node::new(value)));
                }
                
            }
        }
    }
}

pub(crate) struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: PartialEq + std::cmp::Ord> BinaryTree<T> {
    pub(crate) fn new() -> BinaryTree<T> {
        BinaryTree { root: None }
    }

    pub(crate) fn insert(&mut self, value: T) {
        if let Some(root) = &mut self.root {
            root.insert(value);
        } else {
            self.root = Some(Box::new(Node::new(value)));
        }
    }
}



impl<T: std::fmt::Debug> std::fmt::Debug for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack = vec![(&self.root, 0)];
        let mut dir = String::new();
        // iterate through the stack
        while let Some((node, level)) = stack.pop() {
            match node {
                // check if the node exists
                Some(n) => {
                    for _ in 0..level {
                        write!(dir, "    ")?;
                    }
                    write!(dir, "├── {:?}\n", n.value)?;
                    stack.push((&n.right, level + 1));
                    stack.push((&n.left, level + 1));
                }
                None => {
                    // for _ in 0..level {
                    //     write!(dir, "    ")?;
                    // }
                    // write!(dir, "└── null\n")?;
                }
            }
        }
        write!(f, "BinaryTree:\n{}", dir)
    }
}
