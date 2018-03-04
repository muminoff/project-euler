const MAX: u32 = 100;
fn main() {
    let sq_sum: u32 = (1..MAX + 1).map(|x| x * x).sum();
    let mut sum: u32 = (1..MAX + 1).sum();
    sum *= sum;
    let diff: u32 = sum - sq_sum;
    println!("sum of squares {} => {}", MAX, sq_sum);
    println!("square of sum => {}", sum);
    println!("{} - {} = {}", sum, sq_sum, diff);
}
