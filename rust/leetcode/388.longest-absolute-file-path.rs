/*
 * @lc app=leetcode id=388 lang=rust
 *
 * [388] Longest Absolute File Path
 */

// @lc code=start
impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let s = input.split('\n').map(|x| (x.len(),x.matches("\t").count(),x.matches(".").count())).collect::<Vec<(usize,usize,usize)>>();
        let mut max = 0;
        let mut levels =Vec::new();
        for n in &s{
            if n.1<levels.len(){
                levels[n.1]=n.0-n.1;
            }else {
                levels.push(n.0-n.1);
            }
            if n.2>0{
                let m = levels[..=n.1].iter().sum::<usize>()+n.1;
                if m>max{
                    max = m;
                }
            }
        }

        max as i32
    }
}
// @lc code=end
