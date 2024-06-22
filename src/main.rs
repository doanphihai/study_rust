mod guess_word;

fn main() {
    let a = [111, 222, 333];
    println!("Hello, world! {:?}", a);
    let a = [3; 5];
    println!("Hello, world! {:?}", a);
    guess_word::guess_word();
}
