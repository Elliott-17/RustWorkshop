// TODO: Fix the compiler error.

fn make_greeting(name: &str) -> &String {
    let greeting = String::from("Hello, ") + name;
    &greeting
}

fn main() {
    let g = make_greeting("Bob");
    println!("{g}");
}
