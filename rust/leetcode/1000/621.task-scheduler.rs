/*
 * @lc app=leetcode id=621 lang=rust
 *
 * [621] Task Scheduler
 */

// @lc code=start
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if n==0{
        return tasks.len() as i32;
        }
        let mut m = std::collections::HashMap::new();
        for t in &tasks{
            *m.entry(*t).or_insert(0)+=1;
        }
        let max = *m.iter().max_by_key(|x| x.1).unwrap().1;
        let maxcount = m.iter().filter(|x|*x.1==max).count() as i32;
        ((max-1)*(n+1)+maxcount).max(tasks.len() as i32)
    }
}
// @lc code=end

