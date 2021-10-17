fn main() {
    let mut answer = 0;
    for a in 3..=998 {
        for b in 4..=997 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                let abc = a * b * c;
                if abc > answer {
                    answer = abc;
                }
            }
        }
    }
    println!("{}", answer);
}
