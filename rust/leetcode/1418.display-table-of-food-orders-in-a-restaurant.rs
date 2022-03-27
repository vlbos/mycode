/*
 * @lc app=leetcode id=1418 lang=rust
 *
 * [1418] Display Table of Food Orders in a Restaurant
 */

// @lc code=start
impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut m = HashMap::new();
        let mut s = HashSet::new();
        for o in &orders{
            *m.entry(o[1].clone()).or_insert(HashMap::new()).entry(o[2].clone()).or_insert(0)+=1;
            s.insert(o[2].clone());
        }
        let mut ans = Vec::new();
        let mut sv:Vec<String> = s.iter().cloned().collect();
        sv.sort();
        for (k,v) in &m{
            let mut tmp = Vec::new();
            tmp.push(k.parse::<i32>().unwrap());
            for i in &sv{
                if let Some(q)=v.get(i){
                    tmp.push(*q);
                }else{
                    tmp.push(0);
                }
            }
            ans.push(tmp);
        }
        ans.sort();
        let mut ans:  Vec<Vec<String>>= ans.iter().map(|x|x.iter().map(|a|a.to_string()).collect::<Vec<String>>()).collect();
        sv.insert(0,"Table".to_string());
        ans.insert(0,sv);
        ans
    }
}
// @lc code=end

