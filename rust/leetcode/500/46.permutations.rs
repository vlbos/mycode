/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 */

// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn back_tracking(
            mut ans: &mut Vec<Vec<i32>>,
            mut combine: &mut Vec<i32>,
            idx: usize,
        ) {
            if idx == combine.len() {
                // if !ans.contains(&combine) {
                    ans.push(combine.clone());
                // }
                // combine.clear();
                return;
            }
            for i in idx..combine.len(){
                if i!=idx{
                    let t = combine[i];
                    combine[i]=combine[idx];
                    combine[idx]=t;
                }
                back_tracking(ans, combine, idx + 1);
                if i!=idx{
                    let t = combine[i];
                    combine[i]=combine[idx];
                    combine[idx]=t;
                }
            }
          
        }
        let mut ans = Vec::new();
        let mut combine = nums.clone();
        back_tracking( &mut ans, &mut combine, 0);
        ans
    }
}
// @lc code=end
