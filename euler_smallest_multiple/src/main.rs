fn main() {
    let mut collection = Vec::new();
    for x in 100_000_000..300_000_000 {
        let numbers = get_numbers(x);
        let ok: [bool; 20] = [true; 20];
        if numbers == ok {
            collection.push(x);
            // println!("{} => {:?}", x, numbers);
        }
    }
    println!("{:?}", collection.iter().max());
}

fn get_numbers(x: u64) -> Vec<bool> {
    let mut oks: Vec<bool> = Vec::new();
    for i in 1..21 {
        let y = x % i == 0;
        oks.push(y);
    }
    oks
}
