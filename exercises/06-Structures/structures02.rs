struct MyVec{
    data : [u8; 128],
    len: usize,
}

impl MyVec {
    // NOTE: using mut!
    fn vec_push(mut self, c : u8) -> MyVec {
        todo!()
    }

    fn print_str(self) {
        println!("{}", std::str::from_utf8(&v.data[0..v.len]).unwrap());
    }
}

fn main() {
    let mut v : MyVec = todo!();

    for c in "STACS is cool".bytes() {
        v = vec_push(v, c);
    }

    print_str(v);
}
