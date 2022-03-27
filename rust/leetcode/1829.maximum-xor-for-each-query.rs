/*
 * @lc app=leetcode id=1829 lang=rust
 *
 * [1829] Maximum XOR for Each Query
 */

// @lc code=start
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut x = 0;
        for &v in &nums {
            x ^= v;
        }
        let b2 = (1 << maximum_bit)-1;
        let mut ans = Vec::new();
        for &v in nums.iter().rev() {
            ans.push(x^b2);
            x ^= v;
        }
        ans 
    }
}
// @lc code=end
