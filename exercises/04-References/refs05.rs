// TODO: Fix the compiler error.

fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(&s1, &s2);
        println!("longest: {result}");
    }
}
