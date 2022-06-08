/*
 * @lc app=leetcode id=1467 lang=rust
 *
 * [1467] Probability of a Two Boxes Having The Same Number of Distinct Balls
 */

// @lc code=start
impl Solution {
    pub fn get_probability(balls: Vec<i32>) -> f64 {
        let mut f = vec![vec![1; 7]; 8];
        let mut t = balls.iter().sum::<i32>();
        t /= 2;
        let n = balls.len();
        for i in (0..n).rev() {
            for j in 1..=balls[i] {
                f[i][j as usize] = f[i][j as usize - 1] * (balls[i] - j + 1) / j;
            }
        }
        fn dfs(
            balls: &Vec<i32>,
            start: usize,
            a: i32,
            b: i32,
            c: i64,
            d: i32,
            t: i32,
            f: &Vec<Vec<i32>>,
            ans: &mut Vec<i64>,
        ) {
            if start == balls.len() {
                if a != t {
                    return;
                }
                ans[1] += c;
                if b == d {
                    ans[0] += c;
                }
                return;
            }
            dfs(balls, start + 1, a, b, c, d, t, f, ans);
            for i in 1..=balls[start] {
                if a + i > t {
                    break;
                }
                dfs(
                    balls,
                    start + 1,
                    a + i,
                    b + 1,
                    c * f[start][i as usize] as i64,
                    if i == balls[start] { d - 1 } else { d },
                    t,
                    f,
                    ans,
                );
            }
        }
        let mut ans = vec![0, 0];
        dfs(&balls, 0, 0, 0, 1, balls.len() as i32, t, &f, &mut ans);
        (ans[0] as f64) / (ans[1] as f64)
    }
}
// @lc code=end
