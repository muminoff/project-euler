fn main() {
    let s: u32 = 1_000;
    let upto: u32 = (s - 3) / 3;
    for a in 1..upto + 1 {
        let upto2: u32 = (s-1-a) / 2;
        for b in a+1..upto2 {
            let c = s-a-b;
            if c*c == a*a + b*b {
                println!("{}, {}, {} => {}", a, b, c, (a*b*c));
            }
        }
    }
}
