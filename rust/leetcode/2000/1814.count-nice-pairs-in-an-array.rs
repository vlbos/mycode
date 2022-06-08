/*
 * @lc app=leetcode id=1814 lang=rust
 *
 * [1814] Count Nice Pairs in an Array
 */

// @lc code=start
impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
       let rev = nums
            .iter()
            .map(|x| {
                x.to_string()
                    .chars()
                    .rev()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap()
            })
            .collect::<Vec<i32>>();
        let mut ans = 0;
        let mut m = std::collections::HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            let r = v - rev[i];
            if let Some(c) = m.get_mut(&r) {
                ans += *c;
                ans %= 1000_000_007;
                *c += 1;
            } else {
                m.insert(r, 1);
            }
        }
        ans % 1000_000_007
    }
}
// @lc code=end
