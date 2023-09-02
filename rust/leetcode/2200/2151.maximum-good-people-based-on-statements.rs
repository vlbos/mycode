/*
 * @lc app=leetcode id=2151 lang=rust
 *
 * [2151] Maximum Good People Based on Statements
 */

// @lc code=start
impl Solution {
    pub fn maximum_good(statements: Vec<Vec<i32>>) -> i32 {
        let n = statements.len();
        let mut ans = 0;
        let check = |mask: i32| {
            for i in 0..n {
                for j in 0..n {
                    if i == j {
                        continue;
                    }
                    if statements[i][j] == 0 && mask & (1 << i) > 0 && mask & (1 << j) > 0
                        || statements[i][j] == 1 && mask & (1 << i) > 0 && mask & (1 << j) == 0
                    {
                        return false;
                    }
                }
            }
            true
        };
        for mask in 1..(1 << n) {
            if check(mask) {
                ans = ans.max(mask.count_ones())
            }
        }
        ans as _
    }
}
// @lc code=end
impl Solution {
    pub fn maximum_good(statements: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        fn back_tracking(idx:usize,statements: &Vec<Vec<i32>>,good:HashSet<usize>,mut bad:HashSet<usize>,ans:&mut i32){
            if idx==statements.len(){
                *ans=(*ans).max(good.len() as i32);
                return
            }
            for i in idx..statements.len(){
                if bad.contains(&i){
                    continue
                }
                let mut new_good:HashSet<usize>=statements[i].iter().enumerate().filter_map(|(k,&x)| if x==1{Some(k)}else{None}).collect();
                new_good.insert(i);
                let mut new_bad:HashSet<usize>=statements[i].iter().enumerate().filter_map(|(k,&x)| if x==0{Some(k)}else{None}).collect();
                new_good=new_good.difference(&good).cloned().collect();
                new_bad=new_bad.difference(&bad).cloned().collect();
                let all_good:HashSet<_>=new_good.union(&good).cloned().collect();
                let all_bad:HashSet<_>=new_bad.union(&bad).cloned().collect();
                if all_good.intersection(&all_bad).count()==0{
                    back_tracking(idx+1,statements,all_good,all_bad,ans);
                }
                if !bad.contains(&i){
                    bad.insert(i);
                }
            }
                if good.intersection(&bad).count()==0{
                   *ans=(*ans).max(good.len() as i32);
                }
        }
        let mut ans=0;
        back_tracking(0,&statements,HashSet::new(),HashSet::new(),&mut ans);
        ans
    }
}