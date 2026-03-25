// TODO: Fix the compiler error.

fn add_item(v: &mut Vec<i32>, item: i32) {
    v.push(item);
}

fn main() {
    let mut numbers = vec![1, 2, 3];
    add_item(numbers, 4);
    println!("{numbers:?}");
}
