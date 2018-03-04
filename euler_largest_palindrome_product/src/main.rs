fn main() {
    let mut numbers: Vec<u64> = Vec::new();
    for i in 111..1_000 {
        for k in 111..1_000 {
            let z = i * k;
            if is_palindrome(z) {
                numbers.push(z);
            }
        }
    }
    println!("{}", numbers.iter().max().unwrap());
}

fn is_palindrome(x: u64) -> bool {
    let original = x.to_string();
    let reversed = original.chars().rev().collect::<String>();
    original == reversed
}
