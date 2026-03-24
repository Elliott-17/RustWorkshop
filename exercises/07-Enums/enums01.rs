enum MyResult {
    MyOk(u32),
    MyErr(i32),
}

use MyResult::*;

fn validate(x : i32) -> MyResult {
    if x >= 0 {
        MyOk(x.try_into().unwrap())
    } else {
        MyErr(x)
    }
}

fn incr_validate(x : i32) -> MyResult {
    // TODO: can't use the ? for MyResult, translate to a match
    let xx = validate(x)?;
    validate(x + 1)
}

fn main() {
    let x = validate(10);
    // TODO: can't print MyResult, use a match to print each variant
    println!("{}", x);
}
