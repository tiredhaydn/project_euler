use crate::bigint::BigUInt;


type Term = (i64, (i64, i64));      //  a * (√n + b)

fn normalize((num, denom): (Term, i64)) -> (Term, i64) {
    let a = crate::utils::gcd(num.0 as u64, denom as u64) as i64;
    ((num.0 / a, num.1), denom / a)
}


//      num           num * (√n - b)
// -------------- -> ----------------
//  a * (√n + b)       a * (n - b^2)
fn rationalize_denom((num, denom): (i64, Term)) -> (Term, i64) {
    let a = denom.0;
    let n = denom.1.0;
    let b = denom.1.1;

    let num = (num, (n, -b));
    let denom = a * (n - b * b);

    normalize((num, denom))
}


//   a * (√n + b)            a * (√n + new_b)
//  -------------- -> int + ------------------
//      denom                     denom
fn convert_to_mixed_frac((num, denom): (Term, i64)) -> (i64, (Term, i64)) {
    let a = num.0;
    let n = num.1.0;
    let b = num.1.1;

    let floor_sqrt_n = (n as f64).sqrt() as i64;
    let (int, new_b) = {
        let mut expelled_num = a * (b + floor_sqrt_n);
        let mut new_b = -floor_sqrt_n;
        while expelled_num % denom != 0 {
            expelled_num -= a;
            new_b += 1;
        }
        (expelled_num / denom, new_b)
    };

    (int, ((a, (n, new_b)), denom))
}


pub fn sqrt_into_continued_frac(n: u32) -> (u32, Vec<u32>) {
    let floor_sqrt_n = (n as f64).sqrt() as i64;
    let a_0 = floor_sqrt_n;                     // 途中から循環するケースが
    let mut period = Vec::new();    // ないと仮定している

    let mut frac: (Term, i64) = ((1, (n as i64, -floor_sqrt_n)), 1);
    let mut remainder_log = vec![frac];

    loop {
        let reciprocal = (frac.1, frac.0);
        let rationalized = rationalize_denom(reciprocal);
        let (integer_part, remainder) = convert_to_mixed_frac(rationalized);
        period.push(integer_part as u32);

        if remainder_log[0] == remainder {          // 途中から循環するケースがないと仮定している
            return (a_0 as u32, period);
        }
        remainder_log.push(remainder);
        frac = remainder;
    }
}


pub fn calc_convergents(a_0: u32, period: &Vec<u32>, n: u32) -> Vec<(BigUInt, BigUInt)> {
    let mut result = vec![(BigUInt::from(a_0), BigUInt::from(1u32))];
    let mut old_denoms = Vec::new();

    for _ in 0..n {
        for &current_denom in period.iter() {
            let mut frac = (BigUInt::from(1u32), BigUInt::from(current_denom));
            for &old_denom in old_denoms.iter().rev() {
                frac.0 += BigUInt::from(old_denom) * frac.1.clone();
                frac = (frac.1, frac.0);
            }
            frac.0 += BigUInt::from(a_0) * frac.1.clone();
            old_denoms.push(current_denom);
            result.push(frac);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_continued_frac() {
        let test_cases: &[(u32, (u32, &[u32]))] = &[
            (2, (1, &[2])),
            (3, (1, &[1, 2])),
            (5, (2, &[4])),
            (6, (2, &[2, 4])),
            (7, (2, &[1, 1, 1, 4])),
            (8, (2, &[1, 4])),
            (10, (3, &[6])),
            (11, (3, &[3, 6])),
            (12, (3, &[2, 6])),
            (13, (3, &[1, 1, 1, 1, 6])),
        ];
        for &(n, (a_0, period)) in test_cases {
            let actual = sqrt_into_continued_frac(n);
            assert_eq!(actual.0, a_0);
            assert_eq!(&actual.1, period);
        }
    }

    #[test]
    fn test_calc_convergents() {
        let test_cases = &[
            ((1, &vec![2], 9), (3363u32, 2378u32)),
            ((1, &vec![2], 1), (3, 2)),
            ((1, &vec![1], 1), (2, 1)),
            ((2, &vec![4], 1), (9, 4)),
            ((2, &vec![2], 1), (5, 2)),
            ((2, &vec![1, 1, 1], 1), (8, 3)),
        ];
        for ((a_0, period, n), (a, b)) in test_cases {
            let actual = calc_convergents(*a_0, period, *n);
            assert_eq!(Some(&(BigUInt::from(*a), BigUInt::from(*b))), actual.last());
        }
    }
}
