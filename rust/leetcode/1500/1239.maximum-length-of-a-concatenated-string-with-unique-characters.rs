/*
 * @lc app=leetcode id=1239 lang=rust
 *
 * [1239] Maximum Length of a Concatenated String with Unique Characters
 */

// @lc code=start
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut masks=vec![0];
        for a in &arr{
            let mut mask = 0;
            for b in a.bytes(){
                let d = b-b'a';
                if mask>>d &1>0{
                    mask=0;
                    break;
                }
                mask|=1<<d;
            }
            if mask==0{
            continue;
            }
            let n = masks.len();
            for i in 0..n{
                let m = masks[i];
                if m&mask==0{
                    let mm :i32= m|mask;
                    masks.push(mm);
                    ans =ans.max(mm.count_ones());
                }
            }
        }
        ans as _
    }
}
// @lc code=end

