// x^2 - D * y^2 = 1の整数解の求め方
// 
// 1. √Dの連分数展開[a_0; a_1, a_2, ..., a_{m-1}, a_m]を求める
// 2. √Dの連分数展開[a_0; a_1, a_2, ..., a_{m-1}, a_m]からa_mを除いた
//    [a_0; a_1, a_2, ..., a_{m-1}]を1回だけ適用して√Dの近似分数P/Qを求める
// 3. (P, Q)がx^2 - D * y^2 = 1の解
//
// 1の工程はproblem064、2の工程はproblem065から流用すればよい

// ただし、循環節の長さが3以上の奇数であるとき、
// 例えば順関節の長さが13のとき、
// x1^2 - 13 * y1^2 = 1の解を上記の方法で求めると
// (x1, y1) = (18, 5)
// 18 * 18 - 13 * 5 * 5 = -1
// となり求められない。
// ここで、
// x + y√13 = (x1 + y1√13)^2
// となるような(x, y)が
// x^2 - D * y^2 = 1
// を満たすため、以下のように計算する。
// (18 + 5√13)^2 = 18 * 18 + 2 * 18 * 5√13 + 25 * 13
//               = 649 + 180√13
// (x, y) = (649, 180)
//
// 一般的には以下のように計算すればよい
//
// (p + q√D)^2 = p^2 + D * q^2 + 2 * p * q√D
//      (x, y) = (p^2 + D * q^2, 2 * p * q)

use project_euler::continued_fraction::*;
use project_euler::bigint::BigUInt;


fn is_square(n: u32) -> bool {
    (n as f64).sqrt().floor().powi(2) as u32 == n
}


fn solve_pells_equation(d: u32) -> (BigUInt, BigUInt) {
    let (a_0, mut repeating_part) = sqrt_into_continued_frac(d);
    if repeating_part.len() > 1 {
        repeating_part.pop();
    }
    let (p, q) = calc_convergents(a_0, &repeating_part, 1).pop().unwrap();
    if repeating_part.len() % 2 == 0 {
        let x = p.clone() * p.clone() + BigUInt::from(d) * q.clone() * q.clone();
        let y = BigUInt::from(2) * p * q;
        (x, y)
    } else {
        (p, q)
    }
}


fn main() {
    let (answer, _) = (2..=1000)
        .filter(|&d| !is_square(d))
        .map(|d| (d, solve_pells_equation(d)))
        .max_by_key(|(_, (x, _))| x.clone())
        .unwrap();
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_pells_equation() {
        use super::BigUInt;

        for n in 2..=1000 {
            if is_square(n) {
                continue;
            }
            let (p, q) = solve_pells_equation(n);
            assert_eq!(p.clone() * p, BigUInt::from(n) * q.clone() * q + BigUInt::from(1));
        }
    }
}
