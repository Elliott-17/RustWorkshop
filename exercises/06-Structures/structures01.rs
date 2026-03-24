struct MyVec{
    data : [u8; 128],
    len: usize,
}

// NOTE: using mut!
fn vec_push(mut v: MyVec, c : u8) -> MyVec {
    todo!()
}

fn print_str(v: MyVec) {
    println!("{}", std::str::from_utf8(&v.data[0..v.len]).unwrap());
}

fn main() {
    let mut v : MyVec = todo!();

    for c in "STACS is cool".bytes() {
        v = vec_push(v, c);
    }

    print_str(v);
}
