/*
 * @lc app=leetcode id=992 lang=rust
 *
 * [992] Subarrays with K Different Integers
 */

// @lc code=start
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let (mut l1, mut l2, mut r) = (0, 0, 0);
        let (mut t1, mut t2) = (0, 0);
        let mut m1 = std::collections::HashMap::new();
        let mut m2 = m1.clone();
        let mut ans = 0;

        while r < nums.len() {
            if *m1.get(&nums[r]).unwrap_or(&0) == 0 {
                t1 += 1;
            }
            *m1.entry(nums[r]).or_insert(0) += 1;
            if *m2.get(&nums[r]).unwrap_or(&0) == 0 {
                t2 += 1;
            }
            *m2.entry(nums[r]).or_insert(0) += 1;
            while t1 > k {
                m1.entry(nums[l1]).and_modify(|x| *x -= 1);
                if *m1.get(&nums[l1]).unwrap_or(&0) == 0 {
                    m1.remove(&nums[l1]);
                    t1 -= 1;
                }
                l1 += 1;
            }
            while t2 > k - 1 {
                m2.entry(nums[l2]).and_modify(|x| *x -= 1);
                if *m2.get(&nums[l2]).unwrap_or(&0) == 0 {
                    m2.remove(&nums[l2]);
                    t2 -= 1;
                }
                l2 += 1;
            }
            ans += l2 - l1;
            r += 1;
        }
        ans as _
    }
}
// @lc code=end
