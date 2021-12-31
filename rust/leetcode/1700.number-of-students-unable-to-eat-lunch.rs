/*
 * @lc app=leetcode id=1700 lang=rust
 *
 * [1700] Number of Students Unable to Eat Lunch
 */

// @lc code=start
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut s = students;
        let mut k = 0;
        let mut cnt = 0;
        let mut len = 0;
        while k < sandwiches.len() {
            cnt = 0;
            len = s.len();
            loop {
                while !s.is_empty() && k < sandwiches.len() && s[0] == sandwiches[k] {
                    s.remove(0);
                    k += 1;
                    cnt = 0;
                }
                if s.is_empty() || k >= sandwiches.len() {
                    break;
                } else {
                    cnt += 1;
                    s.push(s[0]);
                    s.remove(0);
                }
                if cnt == len {
                    break;
                }
            }
            if s.is_empty() ||cnt == len {
                break;
            }
        }
        s.len() as i32
    }
}
// @lc code=end
