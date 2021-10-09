use project_euler::utils::gcd;


fn main() {
    let answer = (1..=20).fold(1, |acc, x| acc * x / gcd(acc, x));
    println!("{}", answer);
}
