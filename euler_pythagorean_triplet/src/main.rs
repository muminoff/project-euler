const MAX: u32 = 2_000;

fn main() {
    for x in 1..1001 {
        // println!("x -> {}", x);
        for y in 1..1001 {
            // println!("y -> {}", y);
            for z in 1..1001 {
                // println!("z -> {}", z);
                if is_valid(vec![x, y, z]) && is_100(vec![x, y, z]) {
                    let product = x * y * z;
                    println!("{}^2 + {}^2 = {}^2 (product -> {})", x, y, z, product);
                }
            }
        }
    }
}

fn square(num: u32) -> u32 {
    num * num
}

fn is_valid(numbers: Vec<u32>) -> bool {
    let x = numbers[0];
    let y = numbers[1];
    let z = numbers[2];
    square(x) + square(y) == square(z)
}

fn is_100(numbers: Vec<u32>) -> bool {
    let x = numbers[0];
    let y = numbers[1];
    let z = numbers[2];
    x + y + z == MAX
}
