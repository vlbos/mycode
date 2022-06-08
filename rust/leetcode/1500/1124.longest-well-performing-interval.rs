/*
 * @lc app=leetcode id=1124 lang=rust
 *
 * [1124] Longest Well-Performing Interval
 */

// @lc code=start
impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = hours.len();
        let mut cur = 0;
        let mut first = vec![i32::MAX;n+2];
        for (i,&v) in hours.iter().enumerate(){
            cur+=if v>8{1}else{-1};
            let i = i as i32;
            if cur>0{
                ans=i +1;
            }else{
                let j = (-cur) as usize;
                ans = ans.max(i-first[j+1]);
                first[j] = first[j].min(i);
            }
        }
        ans
    }
}
// @lc code=end

