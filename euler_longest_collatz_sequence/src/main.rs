use std::collections::BTreeMap;

enum Number {
    Even,
    Odd,
}

fn main() {
    let mut collatz_map = BTreeMap::new();
    for x in 0..1_000_001 {
        let mut num: u64 = x;
        let mut numbers: Vec<u64> = vec![];
        numbers.push(num);
        while num > 1 {
            match check_number(num) {
                Number::Even => process_even(&mut num, &mut numbers),
                Number::Odd => {
                    num = process_odd(&num);
                    numbers.push(num);
                }
            }
        }
        collatz_map.insert(numbers.len(), x);
    }
    println!("{:?}", collatz_map);
}

fn check_number(n: u64) -> Number {
    if n % 2 == 0 {
        Number::Even
    } else {
        Number::Odd
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

// fn main() {
//     let mut map = BTreeMap::new();
//     map.insert(10, 2);
//     map.insert(1, 10);
//     map.insert(0, 214124);
//     println!("{:?}", map);
// }
