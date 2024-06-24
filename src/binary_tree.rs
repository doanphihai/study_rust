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
                // Just want to write this in different ways
                match &mut self.right {
                    Some(right) =>  right.insert(value),
                    None => self.right = Some(Box::new(Node::new(value))),
                }
                
            }
        }
    }

    // search for a value in the tree in a interative way
    
}

pub(crate) struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: PartialEq + std::cmp::Ord + Clone> BinaryTree<T> {
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

    
    pub(crate) fn search(&self, value: &T) -> bool {
        traverse_until(&self.root, |v| v == *value).unwrap()
    }

    pub(crate) fn count_nodes(&self) -> usize {
        traverse(&self.root, 0, 0, |_, c, d| (c, d+1)).unwrap()
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
                    do_print(level, &mut dir, &n.value)?;
                    stack.push((&n.right, level + 1));
                    stack.push((&n.left, level + 1));
                }
                None => {}
            }
        }
        write!(f, "BinaryTree:\n{}", dir)
    }

}

fn do_print<T: std::fmt::Debug>(level: i32, padding: &mut String, n: &T) -> Result<(), std::fmt::Error> {
    for _ in 0..level {
        write!(padding, "    ")?;
    }
    write!(padding, "├── {:?}\n", n)?;
    Ok(())
}

fn traverse<T, D: Clone, C: Clone>(node: &Option<Box<Node<T>>>, acc_value: D, init_value: C, apply: fn(&T, C, D) -> (C, D)) -> Result<D, std::fmt::Error> {
    let mut acc = acc_value.clone();
    let mut stack = vec![(node, init_value)];
    while let Some((n, mut a)) = stack.pop() {
        match n {
            Some(n) => {
                (a, acc) = apply(&n.value, a, acc.to_owned());
                stack.push((&n.right, a.clone()));
                stack.push((&n.left, a));
            }
            None => {}
        }
    }
    Ok(acc)
}

fn traverse_until<T: Clone>(node: &Option<Box<Node<T>>>, apply: impl Fn(T) -> bool) -> Result<bool, std::fmt::Error> {
    let mut stack = vec![node];
    while let Some(n) = stack.pop() {
        match n {
            Some(n) => {
                if apply(n.value.clone()) {
                    return Ok(true);
                }
                stack.push(&n.right);
                stack.push(&n.left);
            }
            None => {}
        }
    }
    Ok(false)
}

