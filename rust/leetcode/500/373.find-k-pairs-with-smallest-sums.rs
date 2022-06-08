/*
 * @lc app=leetcode id=373 lang=rust
 *
 * [373] Find K Pairs with Smallest Sums
 */

// @lc code=start
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for n1 in &nums1 {
            for n2 in &nums2 {
                if ans.len() < k  as usize{
                    ans.push(vec![*n1, *n2]);
                     ans.sort_by(|a,b|(a[0]+a[1]).cmp(&(b[0]+b[1])));
                } else {
                    let l = ans.last().unwrap();
                    if *n1 + *n2 > l[0] + l[1] {
                        break;
                    } else {
                        ans.push(vec![*n1, *n2]);
                       ans.sort_by(|a,b|(a[0]+a[1]).cmp(&(b[0]+b[1])));
                        ans.pop();
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
