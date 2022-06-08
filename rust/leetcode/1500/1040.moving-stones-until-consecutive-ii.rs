/*
 * @lc app=leetcode id=1040 lang=rust
 *
 * [1040] Moving Stones Until Consecutive II
 */

// @lc code=start
impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let mut stones=stones;
        stones.sort();
        let n =stones.len();
        let nn =  n as i32;
        let mut mx = stones[n-1]-stones[0]+1-nn;
        mx -=  (stones[n-1]-stones[n-2]-1).min(stones[1]-stones[0]-1);
        let mut mi =mx;
        let mut j = 0;
        for i in 0..n{
                while j+1<n && stones[j+1]-stones[i]+1<=nn {
                j+=1;
                }
                let mut cost = n-(j-i+1);
                if j-i+1==n-1 && stones[j]-stones[i]+1==nn-1{
                  cost = 2;
                }
                mi = mi.min(cost as i32);
        }
        vec![mi,mx]
    }
}
// @lc code=end

