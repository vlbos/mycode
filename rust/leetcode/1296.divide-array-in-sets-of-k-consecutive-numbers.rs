/*
 * @lc app=leetcode id=1296 lang=rust
 *
 * [1296] Divide Array in Sets of K Consecutive Numbers
 */

// @lc code=start
impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
         let k = k as usize;
        if nums.len() % k > 0 {
            return false;
        }
        let mut nums=nums;
        nums.sort();
        let mut m = std::collections::HashMap::new();
        for &v in &nums {
            *m.entry(v).or_insert(0) += 1;
        }
        for  &v in &nums  {
            if *m.get(&v).unwrap_or(&0) == 0 {
                continue;
            }
            for i in 0..k {
                if let Some(mut n) = m.get_mut(&(v + i as i32)) {
                    if *n > 0 {
                        *n -= 1;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}
// @lc code=end
