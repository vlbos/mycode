/*
 * @lc app=leetcode id=2064 lang=rust
 *
 * [2064] Minimized Maximum of Products Distributed to Any Store
 */

// @lc code=start
impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let check = |x: i32| -> bool {
            let mut ans = 0;
            for &q in &quantities {
                ans += (q - 1) / x + 1;
            }
            ans <= n
        };
        let mut r = *quantities.iter().max().unwrap();
        let mut l = 1;
        while l < r {
            let mid = (l + r) / 2;
            if check(mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}
// @lc code=end
