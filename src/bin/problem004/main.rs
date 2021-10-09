use project_euler::utils::is_palindrome;


fn main() {
    let mut lower_limit = 101;
    let mut answer = lower_limit * lower_limit;
    let mut i = 999;
    while i >= lower_limit {
        // j > iの範囲の(i, j)は既に(j, i)として調べている
        // j < lower_limitの範囲はanswer未満のnしか存在しない
        for j in (lower_limit..=i).rev() {
            let n = i * j;
            // 以下のコメントアウトしているbreakは不要
            // if n <= answer {
            //     break;
            // }
            //
            // 理由:
            // nの最小値がi * lower_limit、
            // answerをa回前の外側ループでの最小値(i + a) * (lower_limit - a)とすると、
            // n > answerが成り立つ条件は、
            // i * lower_limit > (i + a) * (lower_limit - a)
            //
            // ここでa > 0かつ外側ループの条件より
            //               i >= lower_limit
            //               i > lower_limit - a
            //               0 > lower_limit - i - a
            //               0 > a * (lower_limit - i - a)
            // i * lower_limit > i * lower_limit + a * (lower_limit - i - a)
            // i * lower_limit > (i + a) * (lower_limit - a)
            //
            // よって常にn > answerが成り立つから。
            if is_palindrome(n) {
                answer = n;
                lower_limit = j;
                break;
            }
        }
        i -= 1;
        lower_limit += 1;   // answer = i * j > (i - 1) * jなので(i - 1) * (j + 1)まで調べれば十分
    }
    println!("{}", answer);
}
