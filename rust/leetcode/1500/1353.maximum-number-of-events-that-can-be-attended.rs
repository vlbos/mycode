/*
 * @lc app=leetcode id=1353 lang=rust
 *
 * [1353] Maximum Number of Events That Can Be Attended
 */

// @lc code=start
impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let n  = 100002;
        let mut ins=vec![Vec::new();n];
        let mut outs = vec![Vec::new();n];
        let mut bs = std::collections::BTreeSet::new();

        let mut ans = 0;
        let mut max = 0;
        for (i,e) in events.iter().enumerate(){
            let (l,r)=(e[0] as usize,e[1] as usize);
            ins[l].push(i);
            outs[r+1].push(i);
            max = max.max(r);
        }
        for i in 1..=max{
            for &x in &ins[i]{
                bs.insert((events[x][1],x));
            }
            for &x in &outs[i]{
                bs.remove(&(events[x][1],x));
            }
            if !bs.is_empty(){
                let removed = *bs.iter().nth(0).unwrap();
                bs.remove(&removed);
                ans+=1;
            }
        }
        ans
    }
}
// @lc code=end
