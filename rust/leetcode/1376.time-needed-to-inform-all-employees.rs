/*
 * @lc app=leetcode id=1376 lang=rust
 *
 * [1376] Time Needed to Inform All Employees
 */

// @lc code=start
impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut ans = 0;
        for (j, &v) in inform_time.iter().enumerate() {
            if v == 0 {
                let mut i =j as i32;
                let mut tmp = 0;
                while i!=-1{
                        tmp+=inform_time[i as usize];
                        i=manager[i as usize];
                }
                ans = ans.max(tmp);
            }
        }
        ans
    }
}
// @lc code=end
