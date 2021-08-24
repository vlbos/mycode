/*
 * @lc app=leetcode.cn id=217 lang=rust
 *
 * [217] 存在重复元素
 */

// @lc code=start
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
            let mut v = Vec::<i32>::new();
            for i in nums{
                if v.contains(&i){
                    return true;
                }
                else {
                    v.push(i);
                }
            }
            false
    }
}
// @lc code=end

