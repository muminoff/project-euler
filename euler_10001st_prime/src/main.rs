fn main() {
    let mut numbers: Vec<u32> = Vec::new();
    for x in 1..1_000_000 {
        if is_prime(x) {
            numbers.push(x);
        }
    }
    println!("{}", numbers[10000]);
}

fn is_prime(num: u32) -> bool {
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
