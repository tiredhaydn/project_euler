use crate::utils;

pub struct PrimeIterator {
    primes: Vec<u64>,
    index: usize,
    pseudo_prime: PseudoPrimeIterator,
}

impl PrimeIterator {
    pub fn new() -> Self {
        Self {
            primes: vec![2, 3],
            index: 0,
            pseudo_prime: PseudoPrimeIterator::new(),
        }
    }

    fn add_primes(&mut self, mut n: usize) {
        self.primes.reserve(self.primes.len() + n);
        while n > 0 {
            let pseudo_prime = self.pseudo_prime.next().unwrap();
            if !is_prime(pseudo_prime) {
                continue;
            }
            self.primes.push(pseudo_prime);
            n -= 1;
        }
    }
}

impl Iterator for PrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.primes.len() {
            self.add_primes(100);
        }
        let prime = self.primes[self.index];
        self.index += 1;
        Some(prime)
    }
}


pub struct PseudoPrimeIterator {
    val: u64,
    plus_one: bool,
}

impl PseudoPrimeIterator {
    pub fn new() -> Self {
        Self {
            val: 5,
            plus_one: false,
        }
    }
}

impl Iterator for PseudoPrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.val;
        self.val += if self.plus_one { 4 } else { 2 };
        self.plus_one = !self.plus_one;
        Some(result)
    }
}


// ミラーラビン素数判定法
fn miller_rabin_test(n: u64, bases: &[u64]) -> bool {
    // n - 1 = 2^s * tとなるようなs, tを求める
    let mut t = n - 1;
    let s = t.trailing_zeros() as u64;
    t >>= s;

    'test: for a in bases {
        // a^(2^i * t) i: 0..=s-1
        // の全てがpを法として1と合同であれば
        let mut x = utils::modpow(*a, t, n) as u128;      // x *= x;でのオーバーフロー防止のためu128にキャスト
        if x == 1 || x == n as u128 - 1 {
            continue 'test;
        }
        // そうでなければ
        // その中に-1と合同である数があればYes?
        for _ in 1..=s-1 {
            x *= x;
            x %= n as u128;
            if x == n as u128 - 1 {
                continue 'test;
            }
        }
        // そうでなければfalse
        return false;
    }
    true
}


pub fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        _ if n % 2 == 0 => false,
        _  => {
            let bases: &[u64] = match n {
                _ if n < 2047 => &[2],
                _ if n < 1_373_653 => &[2, 3],
                _ if n < 9_080_191 => &[31, 73],
                _ if n < 25_326_001 => &[2, 3, 5],
                _ if n < 3_215_031_751 => &[2, 3, 5, 7],
                _ if n < 4_759_123_141 => &[2, 7, 61],
                _ if n < 1_122_004_669_633 => &[2, 13, 23, 1662803],
                _ if n < 2_152_382_898_747 => &[2, 3, 5, 7, 11],
                _ if n < 3_474_749_660_383 => &[2, 3, 5, 7, 11, 13],
                _ if n < 341_550_071_728_321 => &[2, 3, 5, 7, 11, 13, 17],
                _ if n < 3_825_123_056_546_413_051 => &[2, 3, 5, 7, 11, 13, 17, 19, 23],
                _ => &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37],
            };
            miller_rabin_test(n, bases)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pseudo_prime() {
        let mut pseudo = PseudoPrimeIterator::new();
        let mut x = pseudo.next().unwrap();
        assert!(x % 6 == 5);
        x = pseudo.next().unwrap();
        assert!(x % 6 == 1);
        x = pseudo.next().unwrap();
        assert!(x % 6 == 5);
        x = pseudo.next().unwrap();
        assert!(x % 6 == 1);
    }

    #[test]
    fn test_is_prime() {
        let primes = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 8191];
        assert!(primes.iter().all(|&x| is_prime(x)));

        let composites = &[4, 6, 8, 9, 10, 12, 14, 15, 16, 21, 57];
        assert!(composites.iter().all(|&x| !is_prime(x)));

        assert_eq!((1..=100000).filter(|&x| is_prime(x)).count(), 9592);
    }
}
