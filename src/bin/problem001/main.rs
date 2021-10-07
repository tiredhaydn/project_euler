fn main() {
    let mut answer = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            answer += n;
        }
    }
    println!("{}", answer);
}


fn sum(m: i32, limit: i32) -> i32 {
    let n = (limit - 1) / m;
    n * (n + 1) / 2 * m
}

fn another_solution() {
    const LIMIT: i32 = 1000;
    let answer = sum(3, LIMIT) + sum(5, LIMIT) - sum(15, LIMIT);
    println!("{}", answer);
}
