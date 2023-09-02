/*
 * @lc app=leetcode id=996 lang=rust
 *
 * [996] Number of Squareful Arrays
 */

// @lc code=start
impl Solution {
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut graph = vec![Vec::new(); n];
        for (i, &num) in nums.iter().enumerate() {
            for j in i + 1..n {
                let r = ((num as f64 + nums[j] as f64).sqrt() + 0.5) as i32;
                if r * r == num + nums[j] {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }
        let mut memo = vec![vec![None; 1 << n]; n];
        let mut factorial = vec![0; 20];
        factorial[0] = 1;
        for i in 1..20 {
            factorial[i] = i as i32 * factorial[i - 1];
        }
        fn dfs(
            i: usize,
            visited: usize,
            graph: &Vec<Vec<usize>>,
            memo: &mut Vec<Vec<Option<i32>>>,
        ) -> i32 {
            if visited == (1 << memo.len()) - 1 {
                return 1;
            }
            if let Some(v) = memo[i][visited] {
                return v;
            }
            let mut ans = 0;
            for &nei in &graph[i] {
                if (visited >> nei) & 1 == 0 {
                    ans += dfs(nei, visited | (1 << nei),graph, memo);
                }
            }
            memo[i][visited] = Some(ans);
            ans
        }
        let mut ans = 0;
        for i in 0..n {
            ans += dfs(i, 1 << i, &graph,&mut memo);
        }
        let mut count = std::collections::HashMap::new();
        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;
        }
        for &v in count.values() {
            ans /= factorial[v];
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let n=nums.len();
       
        let n1=1<<n;
        let mut f=vec![vec![0;n];n1];
        for i in 0..n{
            f[1<<i][i]=1;
        }
        let check=|x:i32|{
            let t= (x as f64).sqrt() as i32;
            t*t==x
        };
        for i in 0..n1{
            for j in 0..n{
                if i>>j&1==0{
                    continue
                }
                for k in 0..n{
                    if k==j||i>>k==0{
                        continue
                    }
                    if check(nums[k]+nums[j]){
                        f[i][j]+=f[i^(1<<j)][k];
                    }
                }
            }
        }
         let mut ans=(0..n).map(|i| f[n1-1][i]).sum::<i32>();
         let mut m=std::collections::HashMap::new();
         for &x in &nums{
             *m.entry(x).or_insert(0)+=1;
         }
         let fact=|x:i32|{
             let mut ans=1;
             for i in 1..=x{
                 ans*=i;
             }
             ans
         };
         for &x in m.values(){
             ans/=fact(x);
         }
        ans
    }
}