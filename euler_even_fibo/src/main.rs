const LIMIT: u64 = 4_000_000;

fn fib(n: u64) -> u64 {
    if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let mut fibos = Vec::new();
    for i in 2..41 {
        let x = fib(i);
        if x % 2 == 0 && x < LIMIT {
            fibos.push(x);
        }
    }
    let sum = fibos.iter().fold(0u64, |a, &b| a + b);
    println!("{:?}", sum);
}
