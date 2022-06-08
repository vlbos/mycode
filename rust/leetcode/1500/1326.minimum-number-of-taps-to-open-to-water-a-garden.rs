/*
 * @lc app=leetcode id=1326 lang=rust
 *
 * [1326] Minimum Number of Taps to Open to Water a Garden
 */

// @lc code=start
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut prev: Vec<i32> = (0..=n).collect();
        for i in 0..=n {
            let (l, r) = (
                0i32.max(i - ranges[i as usize]),
                n.min(i + ranges[i as usize]),
            );
            prev[r as usize] = l.min(prev[r as usize]);
        }
        let (mut breakpoint, mut furthest) = (n, i32::MAX);
        let mut ans = 0;
        for i in (1..=n).rev() {
            furthest = furthest.min(prev[i as usize]);
            if i == breakpoint {
                if furthest >= i {
                    return -1;
                }
                breakpoint = furthest;
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
