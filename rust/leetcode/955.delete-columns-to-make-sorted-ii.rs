/*
 * @lc app=leetcode id=955 lang=rust
 *
 * [955] Delete Columns to Make Sorted II
 */

// @lc code=start
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n= strs.len();
        let w = strs[0].len();
        let mut cur = vec![String::new();n];
        let mut ans = 0;
        let is_sorted=|s:&Vec<String>|->bool{
            s.windows(2).all(|a| a[0]<=a[1])
        };
        for i in 0..w{
            let mut cur2 = cur.clone();
            for j in 0..n{
                cur2[j].push(strs[j].chars().nth(i).unwrap());
            }
            if is_sorted(&cur2){
            cur=cur2;
            }else{
            ans+=1;
            }
        }
        ans
    }
}
// @lc code=end

