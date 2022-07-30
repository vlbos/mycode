// 582\. Kill Process
// ==================

// Given **n** processes, each process has a unique **PID (process id)** and its **PPID (parent process id)**.

// Each process only has one parent process, but may have one or more children processes.
// This is just like a tree structure. Only one process has PPID that is 0, which means this process has no parent process.
//  All the PIDs will be distinct positive integers.

// We use two list of integers to represent a list of processes, where the first list contains PID for each process and the second list contains the corresponding PPID.

// Now given the two lists, and a PID representing a process you want to kill, return a list of PIDs of processes that will be killed in the end.
// You should assume that when a process is killed, all its children processes will be killed. No order is required for the final answer.

// **Example 1:**

// **Input:**
// pid =  \[1, 3, 10, 5\]
// ppid = \[3, 0, 5, 3\]
// kill = 5
// **Output:** \[5,10\]
// **Explanation:**
//            3
//          /   \\
//         1     5
//              /
//             10
// Kill 5 will also kill 10.

// **Note:**

// 1.  The given kill id is guaranteed to be one of the given PIDs.
// 2.  n >= 1.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft)

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        // let len = pid.len();
        // if len == 0 {
        //     return vec![];
        // }
        // let mut ps = HashMap::<usize, Vec<usize>>::new();
        // for i in 0..len {
        //     let cpid = pid[i] as usize;
        //     let ccpid = ppid[i] as usize;
        //     ps.entry(ccpid)
        //         .and_modify(|v| v.push(cpid))
        //         .or_insert_with(|| vec![cpid]);
        // }
        // let mut res = vec![];
        // let mut stack = vec![];
        // stack.push(kill as usize);
        // while let Some(n) = stack.pop() {
        //     res.push(n as i32);
        //     if ps.contains_key(&n) {
        //         stack.extend(ps[&n].iter());
        //     }
        // }
        // res
        use std::collections::HashMap;
        let mut g = HashMap::new();
        for (&c, &p) in pid.iter().zip(&ppid) {
            g.entry(p).or_insert(Vec::new()).push(c);
        }

        fn dfs(g: &HashMap<i32, Vec<i32>>, kill: i32, ans: &mut Vec<i32>) {
            ans.push(kill);
            if let Some(ids) = g.get(&kill) {
                for &id in ids {
                    dfs(g, id, ans);
                }
            }
        }
        let mut ans = Vec::new();
        dfs(&g, kill, &mut ans);
        ans
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kill_process_1() {
        assert_eq!(
            Solution::kill_process(vec![1, 3, 10, 5], vec![3, 0, 5, 3], 5),
            [5, 10]
        );
    }
}
