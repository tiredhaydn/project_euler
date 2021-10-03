fn main() {
    let mut answer = 0;
    let mut fib = (1, 2);
    while fib.0 <= 400_0000 {
        answer += fib.1;
        fib = (fib.1, fib.0 + fib.1);
        fib = (fib.1, fib.0 + fib.1);
        fib = (fib.1, fib.0 + fib.1);
    }
    println!("{}", answer);
}
