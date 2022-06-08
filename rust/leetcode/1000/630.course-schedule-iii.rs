/*
 * @lc app=leetcode id=630 lang=rust
 *
 * [630] Course Schedule III
 */

// @lc code=start
impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_by_key(|x| x[1]);
        let mut q = std::collections::BinaryHeap::new();
        let mut total = 0;
        for course in &courses {
            let (ti, di) = (course[0], course[1]);
            if total + ti <= di {
                total += ti;
                q.push(ti);
            } else if !q.is_empty() && *q.peek().unwrap() > ti {
                total -= *q.peek().unwrap() - ti;
                q.pop();
                q.push(ti);
            }
        }
        q.len() as _
    }
}
// @lc code=end
