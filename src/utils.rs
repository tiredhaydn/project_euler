pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}


pub fn is_palindrome(n: u64) -> bool {
    let num_digits = (n as f64).log10() as u32 + 1;
    let mut lt_pow = 10u64.pow(num_digits - 1);
    let mut rt_pow = 1;
    for _ in 0..num_digits / 2 {
        if n / lt_pow % 10 != n / rt_pow % 10 {
            return false;
        }
        lt_pow /= 10;
        rt_pow *= 10;
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(24, 16), 8);
        assert_eq!(gcd(24, 15), 3);
        assert_eq!(gcd(25, 18), 1);
        assert_eq!(gcd(25, 1), 1);
    }

    #[test]
    fn test_is_palindromic() {
        assert!(is_palindrome(1));
        assert!(is_palindrome(22));
        assert!(!is_palindrome(20));
        assert!(is_palindrome(101));
        assert!(!is_palindrome(120));
        assert!(is_palindrome(1221));
        assert!(!is_palindrome(1021));
        assert!(is_palindrome(12321));
        assert!(!is_palindrome(10031));
        assert!(is_palindrome(906609));
        assert!(!is_palindrome(123021));
    }
}
