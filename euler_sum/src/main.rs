#[test]
fn simple_test() {
    assert_eq!(solution(10), 23);
}

/// `solution` function solves the first project euler problem
pub fn solution(max: u64) -> u64 {
    (0..max).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

fn main() {
    let mut numbers = Vec::new();
    for i in 10..100 {
        numbers.push(solution(i));
    }
    println!("numbers -> {:?}", numbers);
}
