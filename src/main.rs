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
    tree.insert(8);
    tree.insert(1);
    tree.insert(4);
    tree.insert(4);
    tree.insert(7);
    let result = tree.count_nodes();
    println!("Result: {}", result);
    println!("Binary Tree: {:?}", tree);
    let test = "The Cons variant needs the size of an i32 plus the space to store the box’s pointer data. The Nil variant stores no values, so it needs less space than the Cons variant. We now know that any List value will take up the size of an i32 plus the size of a box’s pointer data. By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a List value. Figure 15-2 shows what the Cons variant looks like now.";
    let mut tree: BinaryTree<&str> = binary_tree::BinaryTree::new();
    // iterate through words of test
    for word in test.split_whitespace() {
        tree.insert(word)
    }
    println!("Binary Tree: {:?}", tree);
    let result = tree.count_nodes();
    println!("Result: {}", result);
    let result = tree.search(&"Cons");
    println!("Result: {}", result);
}
