use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BigUInt {
    data: Vec<u32>,
}

impl BigUInt {
    const MAX_DATA: u32 = 9999;

    pub fn new(n: u32) -> Self {
        Self {
            data: vec![n],
        }
    }

    pub fn digit_iter(&self) -> DigitIterator {
        DigitIterator {
            big_uint: self,
            current_chunk: 0,
            b: 1,
        }
    }

    fn process_carry(&mut self) {
        let mut carry = 0;
        for chunk in self.data.iter_mut() {
            *chunk += carry;
            if *chunk > Self::MAX_DATA {
                carry = *chunk / (Self::MAX_DATA + 1);
                *chunk %= Self::MAX_DATA + 1;
            } else {
                carry = 0;
            }
        }
        if carry > 0 {
            self.data.push(carry);
        }
    }
}

impl AddAssign for BigUInt {
    fn add_assign(&mut self, rhs: Self) {
        let (mut self_iter, mut rhs_iter) =
            (self.data.iter_mut().peekable(), rhs.data.iter().peekable());
        while let (Some(_), Some(_)) = (self_iter.peek(), rhs_iter.peek()) {
            *self_iter.next().unwrap() += *rhs_iter.next().unwrap();
        }
        if let (None, Some(_)) = (self_iter.peek(), rhs_iter.peek()) {
            self.data.extend(rhs_iter);
        }
        self.process_carry();
    }
}

impl Add for BigUInt {
    type Output = BigUInt;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl MulAssign for BigUInt {
    fn mul_assign(&mut self, rhs: Self) {
        let mut result = vec![0; self.data.len() + rhs.data.len() - 1];
        for i in 0..rhs.data.len() {
            for j in 0..self.data.len() {
                result[i + j] += self.data[j] * rhs.data[i];
            }
        }
        self.data = result;
        self.process_carry();
    }
}

impl Mul for BigUInt {
    type Output = BigUInt;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.data.len() != other.data.len() {
            return self.data.len().partial_cmp(&other.data.len());
        }
        for (a, b) in self.data.iter().rev().zip(other.data.iter().rev()) {
            if *a != *b {
                return a.partial_cmp(b);
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl From<u32> for BigUInt {
    fn from(x: u32) -> Self {
        Self { data: vec![x] }
    }
}

impl From<&[u32]> for BigUInt {
    fn from(s: &[u32]) -> Self {
        Self { data: s.to_vec() }
    }
}


pub struct DigitIterator<'a> {
    big_uint: &'a BigUInt,
    current_chunk: usize,
    b: u32,
}

impl<'a> Iterator for DigitIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.b > BigUInt::MAX_DATA {
            self.current_chunk += 1;
            self.b = 1;
        }
        if self.current_chunk >= self.big_uint.data.len() {
            return None;
        }
        let result = self.big_uint.data[self.current_chunk] / self.b % 10;
        self.b *= 10;
        Some(result)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    const M0: u32 = BigUInt::MAX_DATA;
    const M1: u32 = M0 - 1;

    #[test]
    fn test_addassign() {
        let test_cases: &[(&[u32], &[u32], &[u32])]  = &[
            (&[0], &[0], &[0]),
            (&[0], &[1], &[1]),
            (&[1], &[1], &[2]),
            (&[1], &[M0], &[0, 1]),
            (&[1], &[M0, M0], &[0, 0, 1]),
            (&[M0, M0], &[1], &[0, 0, 1]),
            (&[M0], &[M0], &[M1, 1]),
            (&[M0, M0], &[M0, M0], &[M1, M0, 1]),
            (&[1, 1, 1], &[M0, M0], &[0, 1, 2]),
            (&[2, 2, 1], &[M0, M1], &[1, 1, 2]),
            (&[1, 2, 2, 1], &[M0, M1], &[0, 1, 3, 1]),
        ];
        for (_i, &(a, b, expected)) in test_cases.iter().enumerate() {
            let (mut a, b) = (BigUInt::from(a), BigUInt::from(b));
            a += b;
            assert_eq!(&a.data, expected);
        }
    }

    #[test]
    fn test_mulassign() {
        let test_cases: &[(&[u32], &[u32], &[u32])]  = &[
            (&[1, 2, 3], &[2], &[2, 4, 6]),
            (&[2], &[1, 2, 3], &[2, 4, 6]),
            (&[1, 2, 3], &[M0], &[M0, M1, M1, 2]),
            (&[M0], &[1, 2, 3], &[M0, M1, M1, 2]),
            (&[M0], &[M0], &[1, M1]),
            (&[M0, M0, M0], &[2], &[M1, M0, M0, 1]),
            (&[2], &[M0, M0, M0], &[M1, M0, M0, 1]),
            (&[M0, M0], &[M0, M0], &[1, 0, M1, M0]),
            (&[M0, M0, M0], &[M0, M0], &[1, 0, M0, M1, M0]),
            (&[M0, M0], &[M0, M0, M0], &[1, 0, M0, M1, M0]),
            (&[M0, M0, M0], &[M0, M0, M0], &[1, 0, 0, M1, M0, M0]),
        ];
        for &(a, b, expected) in test_cases {
            let (mut a, b) = (BigUInt::from(a), BigUInt::from(b));
            a *= b;
            assert_eq!(&a.data, expected);
        }
    }

    #[test]
    fn test_digit_iterator() {
        let test_cases: &[(&[u32], &[u32])]  = &[
            (&[0], &[0, 0, 0, 0]),
            (&[0, 0], &[0, 0, 0, 0, 0, 0, 0, 0]),
            (&[0, 1], &[0, 0, 0, 0, 1, 0, 0, 0]),
            (&[1, 1], &[1, 0, 0, 0, 1, 0, 0, 0]),
            (&[1, M0], &[1, 0, 0, 0, 9, 9, 9, 9]),
        ];
        for &(data, expected) in test_cases {
            let actual = BigUInt::from(data).digit_iter().collect::<Vec<u32>>();
            assert_eq!(&actual, expected);
        }
    }

    #[test]
    fn test_partial_cmp() {
        use std::cmp::Ordering;

        let test_cases: &[(&[u32], &[u32], Ordering)]  = &[
            (&[0], &[0, 0, 0, 1], Ordering::Less),
            (&[0, 3, 1], &[0, 1], Ordering::Greater),
            (&[0, 1, 2], &[0, 2, 2], Ordering::Less),
            (&[0, 1, 1], &[0, 1, 2], Ordering::Less),
            (&[1, 1, 2], &[0, 1, 2], Ordering::Greater),
            (&[1, 1, 2], &[1, 1, 2], Ordering::Equal),
        ];
        for &(a, b, expected) in test_cases {
            let actual = BigUInt::from(a).partial_cmp(&BigUInt::from(b));
            assert_eq!(actual, Some(expected));
        }
    }
}
