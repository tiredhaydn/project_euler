fn main() {
    let mut n = 600851475143u64;
    let mut i = 7;

    loop {
        while n % i == 0 {
            n /= i;
        }
        if i > n {
            break;
        }
        i += 4;
        while n % i == 0 {
            n /= i;
        }
        if i > n {
            break;
        }
        i += 2;
    }
    println!("{}", i);
}
