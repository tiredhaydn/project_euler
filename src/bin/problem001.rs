fn main() {
    let mut answer = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            answer += n;
        }
    }
    println!("{}", answer);
}
