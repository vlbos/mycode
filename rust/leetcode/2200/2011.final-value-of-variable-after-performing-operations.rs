/*
 * @lc app=leetcode id=2011 lang=rust
 *
 * [2011] Final Value of Variable After Performing Operations
 */

// @lc code=start
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
       operations.into_iter().map(|x|if x.contains("++"){1}else{-1}).reduce(|acc,x| acc+x).unwrap()
    }
}
// @lc code=end

