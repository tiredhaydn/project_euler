use project_euler::prime::PrimeIterator;


fn main() {
    let answer = PrimeIterator::new().take_while(|&p| p < 200_0000).sum::<u64>();
    println!("{}", answer);
}
