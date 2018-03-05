const MAX: u64 = 2_000_000;
fn main() {
    println!("Generating prime numbers upto {}...", MAX);
    let numbers: Vec<u64> = generate_prime_numbers(MAX);
    let sum: u64 = numbers.into_iter().sum();
    println!("{}", sum);
}

fn generate_prime_numbers(max: u64) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();
    for x in 1..max + 1 {
        if is_prime(x) {
            numbers.push(x);
        }
    }
    numbers
}

fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }

    if num % 2 == 0 {
        return num == 2;
    }

    let mut k = 3;
    while k * k <= num {
        if num % k == 0 {
            return false;
        }
        k += 2;
    }

    true
}
