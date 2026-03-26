enum Expr {
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
    Div(i32, i32),

}

fn eval(e : Expr) -> i32 {
    todo!()
}


fn main() {
    use Expr::*;

    println!("{}", eval(Add(1, 2)));
    println!("{}", eval(Sub(3, 4)));
    println!("{}", eval(Mul(5, 6)));
    println!("{}", eval(Div(8, 4)));
}
