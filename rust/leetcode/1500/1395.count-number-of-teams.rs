/*
 * @lc app=leetcode id=1395 lang=rust
 *
 * [1395] Count Number of Teams
 */

// @lc code=start
impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let n = rating.len();
        let mut v = vec![vec![0;2];n];
        let mut ans = 0;
        for i in 1..n{
            for j in 0..i{
                if rating[i]<rating[j]{
                    ans+=v[j][0];
                    v[i][0]+=1;
                }
                if rating[i]>rating[j]{
                    ans+=v[j][1];
                    v[i][1]+=1;
                }
            }
        }
        ans
    }
}
// @lc code=end

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
let mut disc=rating.clone();
        disc.sort();
        let disc:std::collections::HashMap<i32,i32>=disc.into_iter().enumerate().map(|(i,v)| (v,i as i32+1)).collect();
        let n=rating.len();
        let (mut iless,mut imore,mut kless,mut kmore)=(vec![0;n],vec![0;n],vec![0;n],vec![0;n]);
       
        let add=|mut i:i32,v:i32,c:&mut Vec<i32>|{
            let mut ans=0;
            while i<c.len() as i32{
                c[i as usize]+=v;
                i+=-i&i;
            }
            ans
        };
        let get=|mut i:i32,c:&Vec<i32>|{
            let mut ans=0;
            while i>0{
                ans+=c[i as usize];
                i&=i-1;
            }
            ans
        };
        let mut c=vec![0;n+1];

        for (i,r) in rating.iter().enumerate(){
            let id=disc[r];
            iless[i]=get(id,&c);
            imore[i]=get(n as i32,&c)-get(id,&c);
            add(id,1,&mut c);
        }
         let mut c=vec![0;n+1];
          for (i,r) in rating.iter().enumerate().rev(){
            let id=disc[r];
            kless[i]=get(id ,&c);
            kmore[i]=get(n as i32,&c)-get(id,&c);
            add(id,1,&mut c);
        }
        iless.into_iter().zip(kmore).zip(imore.into_iter().zip(kless)).map(|((il,mk),(im,kl))| il*mk+im*kl).sum::<i32>()
    }
}