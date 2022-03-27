/*
 * @lc app=leetcode id=1604 lang=rust
 *
 * [1604] Alert Using Same Key-Card Three or More Times in a One Hour Period
 */

// @lc code=start
impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut m = std::collections::HashMap::new();
        for (i,v) in key_name.iter().enumerate(){
            let hm= key_time[i].split(':').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            m.entry(v).or_insert(Vec::new()).push(hm[0]*60+hm[1]);
        }
        let mut ans = Vec::new();
        for (k,v) in &mut m{
            v.sort();
            if v.windows(3).any(|x|x[2]-x[0]<=60){
                ans.push((*k).clone());
            }
        }
        ans.sort();
        ans
    }
}
// @lc code=end

