/*
 * @lc app=leetcode id=1552 lang=rust
 *
 * [1552] Magnetic Force Between Two Balls
 */

// @lc code=start
impl Solution {
    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let n = position.len();
        let mut position = position;
        position.sort();

        let check = |x: i32| -> bool {
            let mut pre = position[0];
            let mut ans = 1;
            for &p in position.iter().skip(1) {
                if p - pre >= x {
                    pre = p;
                    ans += 1;
                }
            }
            ans >= m
        };
        let (mut l, mut r) = (1, position[n - 1] - position[0]);
        let mut ans = -1;
        while l <= r {
            let mid = (l + r) / 2;
            if check(mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        ans
    }
}
// @lc code=end
