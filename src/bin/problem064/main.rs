use project_euler::continued_fraction::sqrt_into_continued_frac;

fn is_square(n: u32) -> bool {
    (n as f32).sqrt().floor().powi(2) as u32 == n
}

fn main() {
    let answer = (2..=10000)
        .filter(|&n| !is_square(n))
        .map(|x| sqrt_into_continued_frac(x))
        .filter(|(_, period)| period.len() % 2 == 1)
        .count();
    println!("{}", answer);
}
