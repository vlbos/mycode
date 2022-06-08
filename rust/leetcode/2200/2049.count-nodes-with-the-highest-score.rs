/*
 * @lc app=leetcode id=2049 lang=rust
 *
 * [2049] Count Nodes With the Highest Score
 */

// @lc code=start
impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let n = parents.len();
        let mut children = vec![Vec::new(); n];
        for i in 1..n {
            children[parents[i] as usize].push(i);
        }
        let mut highest_score = 0;
        let mut cnt = 0;
        fn dfs(
            k: usize,
            children: &Vec<Vec<usize>>,
            highest_score: &mut i64,
            cnt: &mut i32,
        ) -> usize {
            let n = children.len();
            let mut score = 1;
            let mut size = n - 1;
            for &j in &children[k] {
                let sc = dfs(j, children, highest_score, cnt);
                score *= sc as i64;
                size -= sc;
            }
            if k != 0 {
                score *= size as i64;
            }
            if score > *highest_score {
                *highest_score = score;
                *cnt = 1;
            } else if score == *highest_score {
                *cnt += 1;
            }
            n - size
        }
        dfs(0,&children,&mut highest_score,&mut cnt);
        cnt
    }
}
// @lc code=end
