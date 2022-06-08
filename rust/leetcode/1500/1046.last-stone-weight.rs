/*
 * @lc app=leetcode id=1046 lang=rust
 *
 * [1046] Last Stone Weight
 */

// @lc code=start
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        let mut l = stones.len();
        loop{
             l = stones.len();
            if l==1{
            return stones[0];
            }else if l==0{
            return 0;
            }
            stones.sort();
            if stones[l-1]==stones[l-2]{
                stones.remove(l-1);
                stones.remove(l-2);
            }else{
                stones[l-2]=stones[l-1]-stones[l-2];
                stones.remove(l-1);
            }
        }
        0
    }
}
// @lc code=end

