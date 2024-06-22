struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        // check if the value is less than the current node's value
        if value < self.value {
            // check if the left node exists
            if let Some(left) = &mut self.left {
                // insert the value into the left node
                left.insert(value);
            } else {
                // create a new left node and insert the value
                self.left = Some(Box::new(Node::new(value)));
            }
        // check if the value is greater than the current node's value
        } else if let Some(right) = &mut self.right {
            // insert the value into the right node
            right.insert(value);
        } else {
            // create a new right node and insert the value
            self.right = Some(Box::new(Node::new(value)));
        }
    }
}

struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> BinaryTree<T> {
    fn new() -> BinaryTree<T> {
        BinaryTree { root: None }
    }

    fn insert(&mut self, value: T) {
        if let Some(root) = &mut self.root {
            root.insert(value);
        } else {
            self.root = Some(Box::new(Node::new(value)));
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(8);
    tree.insert(1);
    tree.insert(4);
    tree.insert(7);
    tree.insert(9);
    println!("Binary Tree: {:?}", tree);
}