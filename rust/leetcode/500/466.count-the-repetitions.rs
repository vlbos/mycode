/*
 * @lc app=leetcode id=466 lang=rust
 *
 * [466] Count The Repetitions
 */

// @lc code=start
impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let (mut s1cnt, mut index, mut s2cnt) = (0, 0, 0);
        let mut recall = std::collections::HashMap::<usize,Vec<i32>>::new();
        let bs2 = s2.as_bytes();
        let mut pre_loop;
        let mut in_loop;
        loop {
            s1cnt += 1;
            for b in s1.bytes() {
                if b == bs2[index] {
                    index += 1;
                    if index == bs2.len() {
                        index = 0;
                        s2cnt += 1;
                    }
                }
            }
            if s1cnt == n1 {
                return s2cnt / n2;
            }
            if recall.contains_key(&index) {
                let prime = recall.get(&index).unwrap();
                pre_loop = prime.clone();
                in_loop = vec![s1cnt - prime[0], s2cnt - prime[1]];
                break;
            }
            recall.insert(index, vec![s1cnt, s2cnt]);
        }
        let mut ans = pre_loop[1] + (n1 - pre_loop[0]) / in_loop[0] * in_loop[1];
        let rest = (n1 - pre_loop[0]) % in_loop[0];
        for _ in 0..rest {
            for b in s1.bytes() {
                if b == bs2[index] {
                    index += 1;
                    if index == bs2.len() {
                        index = 0;
                        ans += 1;
                    }
                }
            }
        }
        ans / n2
    }
}
// @lc code=end
impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let (len1,len2)=(s1.len(),s2.len());
        let (bs1,bs2)=(s1.as_bytes(),s2.as_bytes());
        let mut dp=vec![0;len2];
        for i in 0..len2{
            let mut j=i;
            for k in  0..len1{
                if bs1[k]==bs2[j]{
                    j=(j+1)%len2;
                    dp[i]+=1;
                }
            }
        }
        let (mut cnt,mut j)=(0,0);
        for _ in 0..n1{
            let add=dp[j] as usize;
            j=(j+add)%len2;
            cnt+=add;
        }
        ((cnt/len2) as i32 )/n2
    }
}