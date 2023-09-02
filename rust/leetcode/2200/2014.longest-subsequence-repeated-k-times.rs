/*
 * @lc app=leetcode id=2014 lang=rust
 *
 * [2014] Longest Subsequence Repeated k Times
 */

// @lc code=start
impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let check = |t: &String| {
            let bt = t.as_bytes();
            let mut cnt = 0;
            let mut j = 0;
            let (sn, tn) = (s.len(), t.len());
            for (i, b) in s.bytes().enumerate() {
                if tn * (k - cnt) as usize + i - j > sn {
                    return false;
                }
                if b != bt[j] {
                    continue;
                }
                j += 1;
                if j < tn {
                    continue;
                }
                cnt += 1;
                if cnt == k {
                    return true;
                }
                j = 0;
            }
            false
        };
        let mut vs = vec![Vec::new(); 8];
        vs[0].push(String::new());
        for i in 0..vs.len()-1 {
            for k in 0..vs[i].len() {
                for j in 0..26 {
                    let mut r = vs[i][k].clone();
                    r.push((b'a' + j) as char);
                    if check(&r) {
                        vs[i + 1].push(r);
                    }
                }
            }
        }
        for x in vs.iter().rev() {
            if !x.is_empty() {
                return x.iter().max().unwrap().clone();
            }
        }
        String::new()
    }
}
// @lc code=end
impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let mut cnt=vec![0;26];
        for b in s.bytes(){
            cnt[(b-b'a') as usize]+=1;
        }
        let mut valid=String::new();
        for i in (0..26).rev(){
            if cnt[i]>=k{
                valid.push((b'a'+i as  u8) as char);
            }
        }
        fn dfs(i:usize,s: &String,k: i32,cnt:&mut Vec<i32>,valid:&String,mc:&mut Vec<u8>,ans:&mut String){
            let check=|len:usize,mc:&Vec<u8>|{
                    let mut ans=0;
                    let (mut l,mut r)=(0,0);
                    while l<s.len(){
                        while l<s.len() && mc[r]!=s.as_bytes()[l]{
                            l+=1;
                        }
                        if l==s.len(){
                            break
                        }
                        l+=1;
                        r+=1;
                        if r==len{
                            r=0;
                            ans+=1;
                        }
                    }
                    ans>=k
            };
            for b in valid.bytes(){
                mc[i]=b;
                if cnt[(b-b'a') as usize]>=k && check(i+1,&mc){
                           cnt[(b-b'a') as usize]-=k ;
                           if i>=ans.len(){
                               *ans=String::from_utf8(mc[..=i].to_vec()).unwrap();
                           }
                           dfs(i+1,s,k,cnt,valid,mc,ans);
                             cnt[(b-b'a') as usize]+=k ;
                }

            }
        }
        let mut ans=String::new();
        dfs(0,&s,k,&mut cnt,&valid,&mut vec![0;8],&mut ans);
        ans
    }
}