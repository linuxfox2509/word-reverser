fn main() {
    let text = "Hello World";
    let reversed: String = text.chars().rev().collect();
    println!("{}", reversed);
}