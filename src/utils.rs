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


pub fn modpow(a: u64, n: u64, m: u64) -> u64 {
    let a: u128 = a as u128;                // a == u64::MAXの場合のオーバーフロー防止
    let m: u128 = m as u128;                // aに型を合わせるため
    let a_mod_m = a % m;
    if a_mod_m == 0 || a_mod_m == 1 {       // 0と1はn乗してもそのまま
        return a_mod_m as u64;
    }
    if a_mod_m == m - 1 {                   // -1は偶数乗で1奇数乗で奇数乗で-1
        return if n % 2 == 0 { 1 } else { a_mod_m as u64 };
    }
    // n = ∑b_i * 2^i
    // b_i: nの2進数表示のi桁目
    // としたとき、
    // a^n mod m = a^(b_0 * 2^0 + b_1 * 2^1 + b_2 * 2^2 + ... + b_k * 2^k)
    //           = a^b_0 * a^(b_1 * 2) * a^(b_2 * 4) * ... * a^(b_k * 2^k)
    // b_iの値は0または1をとるので1のところでa^(2^i)をかけ合わせればよい。
    let mut modpow = 1;
    let mut a_mod_m = a_mod_m;
    let mut n = n;
    while n != 0 {
        if n & 1 == 1 {                     // ビットが1ならa^(2^i)を蓄積変数にかけ合わせる
            modpow *= a_mod_m;
            modpow %= m;
        }
        a_mod_m *= a_mod_m;                 // a^1, a^2, a^4, a^8, ...と計算する
        a_mod_m %= m;
        if a_mod_m == 1 {
            return modpow as u64;
        }
        n >>= 1;
    }
    modpow as u64
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
