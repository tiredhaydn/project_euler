fn main() {
    let mut answer = 0;
    let mut fib = (1, 2);               // (odd, even)
    while fib.1 <= 400_0000 {
        answer += fib.1;
        fib = (fib.1, fib.0 + fib.1);       // (even, odd)
        fib = (fib.1, fib.0 + fib.1);       // (odd, odd)
        fib = (fib.1, fib.0 + fib.1);       // (odd, even)
    }
    println!("{}", answer);
}
