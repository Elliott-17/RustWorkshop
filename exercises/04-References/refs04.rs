// TODO: Fix the compiler error.

fn main() {
    let mut text = String::from("hello world");

    let first = &text[..5];

    text.push_str("!");

    println!("first word: {first}");
    println!("full text:  {text}");
}
