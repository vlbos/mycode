/*
 * @lc app=leetcode id=1590 lang=rust
 *
 * [1590] Make Sum Divisible by P
 */

// @lc code=start
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len()  as i32;
        let mut ans = n;
        let p = p as i64;
        let sum_mod = nums.iter().map(|&x|x as i64).sum::<i64>() % p;
        if sum_mod==0{
            return 0;
        }
        let mut m = std::collections::HashMap::new();
        m.insert(0, -1);
        let mut sub_mod = 0;
        for (i, &v) in nums.iter().enumerate() {
            sub_mod += v as i64;
            sub_mod %= p;
            let target = (sub_mod - sum_mod + p) % p;
            if let Some(j) = m.get(&target) {
                ans = ans.min(i as i32  - j);
                if ans == 1 && ans != n {
                    return ans;
                }
            }
            m.insert(sub_mod, i as i32);
        }
        if ans == n {
            -1
        } else {
            ans  as _
        }
    }
}
// @lc code=end
