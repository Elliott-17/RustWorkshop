// TODO: Fix the compiler error.

fn print_length(s: &str) {
    println!("\"{}\" is {} characters long", s, s.len());
}

fn main() {
    let name = String::from("Rustacean");
    print_length(name);
    println!("name is still: {name}");
}
