/*
 * @lc app=leetcode id=1424 lang=rust
 *
 * [1424] Diagonal Traverse II
 */

// @lc code=start
impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = std::collections::BTreeMap::new();
        for (i,n) in nums.iter().enumerate(){
            for (j,v) in n.iter().enumerate(){
                ans.entry(i+j).or_insert(Vec::new()).insert(0,*v);
            }
        }
        ans.values().cloned().flatten().collect()
    }
}
// @lc code=end

