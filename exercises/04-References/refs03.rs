// TODO: Fix the compiler error.

fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn main() {
    let scores = vec![10, 20, 30, 40];
    let total = sum(scores);
    println!("scores: {scores:?}");
    println!("total:  {total}");
}
