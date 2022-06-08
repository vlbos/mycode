/*
 * @lc app=leetcode id=1324 lang=rust
 *
 * [1324] Print Words Vertically
 */

// @lc code=start
impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let sv=s.split_ascii_whitespace().collect::<Vec<&str>>();
        let max = sv.iter().map(|x|x.len()).max().unwrap();
        let mut ans = vec![String::new();max];
        for i in 0..max{
            for ss in &sv{
                ans[i].push(if i<ss.len(){ss.chars().nth(i).unwrap()}else{' '});
            }
        }
        ans.iter().map(|x|x.trim_end().to_string()).collect()
    }
}
// @lc code=end

