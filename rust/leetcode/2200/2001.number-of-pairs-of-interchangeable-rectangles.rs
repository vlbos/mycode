/*
 * @lc app=leetcode id=2001 lang=rust
 *
 * [2001] Number of Pairs of Interchangeable Rectangles
 */

// @lc code=start
impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                return a;
            }
            gcd(b, a % b)
        }
        use std::collections::HashMap;
        let mut cnt = HashMap::new();
        for r in &rectangles {
            let c = gcd(r[0], r[1]);
            let (i, j) = (r[0] / c, r[1] / c);
            *cnt.entry(i).or_insert(HashMap::new()).entry(j).or_insert(0) += 1i64;
        }
        let mut ans = 0;
        for (_, vv) in &cnt {
            for (_, &v) in vv {
                ans += v * (v - 1) / 2;
            }
        }
        ans
    }
}
// @lc code=end
