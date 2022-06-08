/*
 * @lc app=leetcode id=2150 lang=rust
 *
 * [2150] Find All Lonely Numbers in the Array
 */

// @lc code=start
impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
         let mut ns = std::collections::HashMap::<i32,i32>::new();
        for &v in &nums{
            *ns.entry(v).or_insert(0)+=1;
        }
        let mut ans = Vec::new();
        for (&k,&v) in &ns {
            if ns.contains_key(&(k + 1)) || ns.contains_key(&(k - 1)) {
                continue;
            }
            if *ns.get(&k).unwrap_or(&0)>1{
                continue;
            }
            ans.push(k);
        }
        ans
    }
}
// @lc code=end
