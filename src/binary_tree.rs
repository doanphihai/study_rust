
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

impl<T: PartialEq + std::cmp::Ord + std::fmt::Debug> std::fmt::Debug for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack = Vec::new();
        stack.push((self.root.as_ref(), 0));
        while let Some((node, level)) = stack.pop() {
            if let Some(node) = node {
                stack.push((node.right.as_ref(), level + 1));
                stack.push((node.left.as_ref(), level + 1));
                write!(f, "\n")?;
                write!(f, "{:indent$}{:?}", "", node.value, indent = level * 2)?;
            }
        }
        Ok(())
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

