/*
 * @lc app=leetcode id=1409 lang=rust
 *
 * [1409] Queries on a Permutation With Key
 */

// @lc code=start
impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut v :Vec<i32>= (1..=m).collect();
        let mut ans = Vec::new();
        for &q in &queries{
            let p = v.iter().position(|x|*x==q);
            if let Some(i) = p{
                ans.push(i as i32);
                let iv = v.remove(i);
                v.insert(0,iv);
            }
        }
        ans
    }
}
// @lc code=end

impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
let (n,m)=(queries.len(),m as usize);
        let mut a=vec![0;m+n+1];
        let update=|mut i:i32,v:i32,a:&mut Vec<i32>|{
            while i<a.len() as i32{
                a[i as usize]+=v;
                i+=-i&i;
            }
        };
         let query=|mut i:i32,a:& Vec<i32>|{
             let mut ans=0;
            while i>0{
                ans+=a[i as usize];
                i&=i-1;
            }
            ans
        };
        let mut pos=vec![0;m+1];
        for i in 1..=m{
            let ni=(n+i) as i32;
            pos[i]=ni;
            update(ni,1,&mut a);
        }
        let mut ans=Vec::new();
        for (i,&v) in queries.iter().enumerate(){
            let mut cur=pos[v as usize];
            update(cur,-1,&mut a);
            ans.push(query(cur,&a));
            let ni=(n-i) as i32;
            pos[v as usize]=ni;
            update(ni,1,&mut a);
        }
        ans
    }
}