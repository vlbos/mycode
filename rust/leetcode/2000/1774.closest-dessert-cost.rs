/*
 * @lc app=leetcode id=1774 lang=rust
 *
 * [1774] Closest Dessert Cost
 */

// @lc code=start
impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut t:Vec<(i32,i32)> = base_costs.iter().map(|x| ((*x - target).abs(), *x)).collect();
        for &c in &topping_costs {
            for (_,b) in t.clone()  {
                t.push((((b + c) - target).abs(), b + c));
                t.push((((b + 2 * c) - target).abs(), b + 2 * c));
            }
        }
        t.sort();
        t[0].1
    }
}
// @lc code=end
impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        fn dfs(p:usize,cur_cost:i32,topping_costs: &Vec<i32>, target: i32,ans:&mut i32){
             if (*ans-target).abs()<cur_cost-target{
                 return
             }
             if (*ans-target).abs()>=(cur_cost-target).abs(){
                 if (*ans-target).abs()>(cur_cost-target).abs(){
                     *ans=cur_cost;
                 }else{
                     *ans=cur_cost.min(*ans);
                 }
             }
             if p==topping_costs.len(){
                 return
             }
             dfs(p+1,cur_cost+topping_costs[p]*2,topping_costs,target,ans);
             dfs(p+1,cur_cost+topping_costs[p],topping_costs,target,ans);
             dfs(p+1,cur_cost,topping_costs,target,ans);
        }
        let mut ans=*base_costs.iter().min().unwrap();
        for &c in &base_costs{
            dfs(0,c,&topping_costs,target,&mut ans);
        }
        ans
    }
}

impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let x=*base_costs.iter().min().unwrap();
        if x>target{
            return x
        }
        let mut can=vec![false;target as usize+1];
        let mut ans=target*2-x;
        for &c in &base_costs{
            if c<=target{
                can[c as usize]=true;
            }else{
                ans=ans.min(c);
            }
        }
        for &c in &topping_costs{
            for count in 0..2{
                for i in (1..=target).rev(){
                    if can[i as usize] && i+c>target{
                        ans=ans.min(i+c);
                    }
                    if i>c &&  !can[i as usize]{
                        can[i as usize]=can[(i-c) as usize];
                    }
                }
            }
        }
        for i in 0..ans-target+1{
            if can[(target-i) as usize]{
                return target-i
            }
        }
        ans
    }
}