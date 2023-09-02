/*
 * @lc app=leetcode id=2003 lang=rust
 *
 * [2003] Smallest Missing Genetic Value in Each Subtree
 */

// @lc code=start
impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = parents.len();
        let mut ans = vec![1; n];
        let mut g = vec![Vec::new(); n];
        for (i, &v) in parents.iter().enumerate().skip(1) {
            g[v as usize].push(i);
        }
        use std::collections::HashMap;
        let mut in_set = HashMap::new();
        fn f(v: i32, nums: &Vec<i32>, g: &Vec<Vec<usize>>, in_set: &mut HashMap<i32, bool>) {
            in_set.insert(nums[v as usize], true);
            for &u in &g[v as usize] {
                if !in_set.contains_key(&nums[u]) {
                    f(u as i32, nums, g, in_set);
                }
            }
        }
        let mut x = if let Some(i) = nums.iter().position(|u| *u == 1) {
            i as i32
        } else {
            -1
        };
        let mut cur = 2;
        while x >= 0 {
            f(x, &nums, &g, &mut in_set);
            while in_set.contains_key(&cur) {
                cur += 1;
            }
            ans[x as usize] = cur;
            x = parents[x as usize];
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n=parents.len();
        let mut f:Vec<i32>=(0..*nums.iter().max().unwrap()+5).collect();
        fn find(x:i32,f:&mut Vec<i32>)->i32{
            let px=f[x as usize];
            if px!=x{
                f[x as usize]=find(px,f);
            }
            f[x as usize]
        }
        let unite=|x:i32,y:i32,f:&mut Vec<i32>|{
            let (px,py)=(find(x,f),find(y,f));
            if px!=py{
                f[px as usize]=py;
            }
        };
        let mut deg=vec![0;n];
        for &p in &parents[1..]{
            deg[p as usize]+=1;
        }
        let mut q=vec![0;n];
        let mut k=0;
        for (i,&d) in deg.iter().enumerate(){
            if d==0{
                q[k]=i as i32;
                k+=1;
            }
        }
        let mut j=0;
        let mut ans=vec![1;n];
        while j!=k{
            let i=q[j] as usize;
            j+=1;
            while find(ans[i],&mut f)==find(nums[i],&mut f){
                ans[i]+=1;
            }
            if i==0{
                    continue
            }
            let p=parents[i] as usize;
            unite(nums[i],nums[p],&mut f);
            ans[p]=ans[p].max(ans[i]);
            deg[p]-=1;
            if deg[p]==0{
                q[k]=p as i32;
                k+=1;
            }
        }
        ans
    }
}