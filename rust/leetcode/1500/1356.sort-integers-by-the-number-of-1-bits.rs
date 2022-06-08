/*
 * @lc app=leetcode id=1356 lang=rust
 *
 * [1356] Sort Integers by The Number of 1 Bits
 */

// @lc code=start
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut r = vec![Vec::new();15];
        for a in &arr{
            let c = a.count_ones();
            r[c as usize].push(*a);
        }
        let mut ans = Vec::new();
        for n in &r{
            if !n.is_empty(){
                let mut nn = n.clone();
                nn.sort();
                ans.extend(nn);
            }
        }
        ans
    }
}
// @lc code=end

