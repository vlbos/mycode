/*
 * @lc app=leetcode id=950 lang=rust
 *
 * [950] Reveal Cards In Increasing Order
 */

// @lc code=start
impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let n = deck.len();
        let mut q= std::collections::VecDeque::new();
        for i in 0..n{
            q.push_back(i);
        }
        let mut deck=deck;
        deck.sort();
        let mut ans = vec![0;n];
        for &d in &deck{
             if let Some(i)=q.pop_front(){
                 ans[i]=d;
             }
             if let Some(i)=q.pop_front(){
                q.push_back(i);
             }
        }
        ans
    }
}
// @lc code=end

