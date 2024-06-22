use binary_tree::BinaryTree;

mod binary_tree;

fn main() {
    let mut tree: BinaryTree<i32> = binary_tree::BinaryTree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(8);
    tree.insert(1);
    tree.insert(4);
    tree.insert(4);
    tree.insert(7);
    tree.insert(9);
    println!("Binary Tree: {:?}", tree);
}
