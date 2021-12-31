/*
 * @lc app=leetcode id=1103 lang=rust
 *
 * [1103] Distribute Candies to People
 */

// @lc code=start
impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let n = num_people as usize;
        let mut ans = vec![0; n].to_vec();
        let mut cs = candies as usize;
        let mut i = 0usize;
        while cs > 0 {
            let index = i % n;
            let c = if cs > i { index + 1 + (i / n) * n } else { cs };
            ans[index] += c as i32;
            cs -= c;
            if n == 0 {
                break;
            }
            i += 1;
        }
        ans
    }
}
// @lc code=end
