/*
 * @lc app=leetcode id=128 lang=rust
 *
 * [128] Longest Consecutive Sequence
 */

// @lc code=start
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let min = *(nums.iter().min().unwrap());
        let max = *(nums.iter().max().unwrap());
        let mut v = vec![false; (max - min + 1) as usize];
        for n in &nums {
            v[(*n - min) as usize] = true;
        }
        let mut ans = 0;
        let mut cnt = 0;
        for n in &v {
            if *n {
                cnt += 1;
            } else {
                if cnt > ans {
                    ans = cnt;
                }
                cnt = 0;
            }
        }
        if cnt > ans {
            ans = cnt;
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let n=nums.len();
        let mut parent:Vec<usize>=(0..n).collect();
        let mut len=vec![1;n];
        fn find(x:usize,parent:&mut Vec<usize>)->usize{
            let px=parent[x];
            if px!=x{
                parent[x]=find(px,parent);
            }
            parent[x]
        }
        let unite=|x:usize,y:usize,parent:&mut Vec<usize>,len:&mut Vec<i32>|{
            let (px,py)=(find(x,parent),find(y,parent));
            if px==py{
                return 
            }
            parent[py]=px;
            len[px]+=len[py];
        };
        let mut map=std::collections::HashMap::new();
        for (i,&num) in nums.iter().enumerate(){
            if map.contains_key(&num){
                continue
            }
            if let Some(&j)=map.get(&(num-1)){
                unite(i,j,&mut parent,&mut len);
            }
            if let Some(&j)=map.get(&(num+1)){
                unite(i,j,&mut parent,&mut len);
            }
            map.insert(num,i);
        }
        len.into_iter().enumerate().map(|(i,v)|  if i==parent[i]{v}else{0}).max().unwrap_or(0)
    }
}