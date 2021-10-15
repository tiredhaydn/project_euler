use project_euler::continued_fraction::calc_convergents;


fn continued_frac_of_e(nth: u32) -> (u32, Vec<u32>) {
    let mut period = Vec::new();
    for i in 1..=nth / 3 {
        period.extend_from_slice(&[1, 2 * i, 1]);
    }
    (2, period)
}


fn main() {
    let (a_0, period) = continued_frac_of_e(100);
    let (num, _) = calc_convergents(a_0, &period, 1).pop().unwrap();
    let answer = num.digit_iter().sum::<u32>();
    println!("{:?}", answer);
}
