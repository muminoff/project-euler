use std::collections::BTreeMap;

#[derive(Debug)]
enum Number {
    Even,
    Odd,
}

use Number::{Even, Odd};

fn main() {
    let mut collatz_map = BTreeMap::new();
    for x in 0..1_000_001 {
        let mut num: u64 = x;
        let mut numbers: Vec<u64> = vec![];
        numbers.push(num); // add first number
        while num > 1 {
            match check_number(num) {
                Even => process_even(&mut num, &mut numbers),
                Odd => {
                    num = process_odd(&num);
                    numbers.push(num);
                }
            }
        }
        collatz_map.insert(numbers.len(), x);
    }
    let (key, value) = collatz_map.iter().next_back().unwrap();
    println!(
        "{:?} number {} has {} chain set.",
        check_number(*value as u64),
        value,
        key
    );
}

fn check_number(n: u64) -> Number {
    match n % 2 {
        0 => Even,
        _ => Odd,
    }
}

fn process_even(num: &mut u64, numbers: &mut Vec<u64>) {
    *num /= 2;
    numbers.push(*num);
}

fn process_odd(num: &u64) -> u64 {
    let x = 3 * num + 1;
    x
}
