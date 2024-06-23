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
    let test = " let mut tree: BinaryTree<String> = binary_tree::BinaryTree::new();";
    let mut tree: BinaryTree<String> = binary_tree::BinaryTree::new();
    // iterate through words of test
    for word in test.split_whitespace() {
        tree.insert(word.to_string())
    }
    println!("Binary Tree: {:?}", tree);
}
