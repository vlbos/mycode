/*
 * @lc app=leetcode id=421 lang=rust
 *
 * [421] Maximum XOR of Two Numbers in an Array
 */

// @lc code=start
impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut x = 0;
        for b in (0..31).rev(){
        let mut h = std::collections::HashSet::new();
              for n in &nums{
                    h.insert(*n>>b);
              }
              let x_next = (x<<1)+1;
              let mut found =false;
              for n in &nums{
                    if h.contains(&((*n>>b)^x_next)){
                        found=true;
                        break;
                    }
              }
              x=x_next;
              if !found{
                x-=1;
              }
        }
        x
    }
}
// @lc code=end

