// TODO: Fix the compiler error.

fn main() {
    let mut s = String::from("hello");

    let r = &s;
    let r_mut = &mut s;
    r_mut.push_str(" world");

    println!("{r}");
}
