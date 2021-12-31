/*
 * @lc app=leetcode id=781 lang=rust
 *
 * [781] Rabbits in Forest
 */

// @lc code=start
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::new();
        for y in &answers{
        *m.entry(*y).or_insert(0)+=1;
        }
        let mut ans =0;
        for (y,x) in &m{
            ans+= (x+y)/(y+1)*(y+1);
        }
        ans
    }
}
// @lc code=end

