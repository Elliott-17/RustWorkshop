use rand::Rng;

// This exercise is about using external crates in Rust.
// You'll need to use the `rand` crate to generate random numbers.

// You will have to make a terminal in vs code and run
// `cargo add rand`
// You may have to modify and resave this file to update the file



fn main() {
    let mut rng = rand::rng();

    let secret_number = rng.next_u32() % 100 + 1;
    println!("The secret number is: {}", secret_number);

}