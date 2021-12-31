/*
 * @lc app=leetcode id=90 lang=rust
 *
 * [90] Subsets II
 */

// @lc code=start
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut freq:Vec<Vec<i32>> = Vec::new();
        for n in &nums {
           
            if freq.is_empty() || freq[freq.len() - 1][0] != *n {
                freq.push(vec![*n, 1]);
            } else {
                let  last = freq.len() - 1;
                freq[last][1] += 1;
            }
        }
        // println!("{:?}",freq);
        let mut ans: Vec<Vec<i32>> = Vec::new();

        fn back_track(
            n: usize,
            freq: &Vec<Vec<i32>>,
            mut ans: &mut Vec<Vec<i32>>,
            mut seq: &mut Vec<i32>,
            idx: usize,
            index: usize,
        ) {
            if idx == n || index==freq.len(){
                ans.push(seq.clone());
                return;
            }
            back_track(n, freq, ans, seq, idx + 1,index+1);
            for _ in 0..freq[index][1] {
                seq.push(freq[index][0]);
                back_track(n, freq, ans, seq, idx + 1,index+1);
            }
            for i in 0..freq[index][1] {
                seq.pop();
            }
        }
        let mut seq = Vec::new();
        back_track(nums.len(), &freq, &mut ans, &mut seq, 0,0);
        ans
    }
}
// @lc code=end
