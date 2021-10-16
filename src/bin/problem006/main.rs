fn main() {
    let n = 100;
    let sum = n * (n + 1) / 2;
    let sum_of_square = sum * (2*n + 1) / 3;
    println!("{}", sum * sum - sum_of_square);
}
