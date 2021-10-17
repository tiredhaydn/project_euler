use project_euler::prime::PrimeIterator;


fn main() {
    let answer = PrimeIterator::new().nth(10000).unwrap();
    println!("{}", answer);
}
