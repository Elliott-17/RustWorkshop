// TODO: Fix the compiler error without changing the `greet` function signature.

fn greet(name: String) {
    println!("Hello, {name}!");
}

fn main() {
    let name = String::from("Alice");
    greet(name);
    println!("Done greeting {name}");
}
