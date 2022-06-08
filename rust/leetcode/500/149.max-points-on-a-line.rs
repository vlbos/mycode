/*
 * @lc app=leetcode id=149 lang=rust
 *
 * [149] Max Points on a Line
 */

// @lc code=start
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 {
            return n as _;
        }
        let mut ans = 0;

        for i in 0..n {
            if ans >= n - i || ans > n / 2 {
                break;
            }
            let mut mp = std::collections::HashMap::new();
            for j in i + 1..n {
                let (mut x, mut y) = (points[i][0] - points[j][0], points[i][1] - points[j][1]);
                if x == 0 {
                    y = 1;
                } else if y == 0 {
                    x = 1;
                } else {
                    if y < 0 {
                        x = -x;
                        y = -y;
                    }
                    let gcd_xy = gcd(x.abs(), y.abs());
                    x /= gcd_xy;
                    y /= gcd_xy;
                }
                *mp.entry(y + 20001 * x).or_insert(0) += 1;
            }
            let max_n = mp.iter().map(|x| *x.1).max().unwrap() + 1;
            ans = ans.max(max_n);
        }
        fn gcd(a: i32, b: i32)->i32 {
            if b == 0 {
                return a;
            }
            gcd(b, a % b)
        }
        ans as _
    }
}
// @lc code=end
