/*
 * @lc app=leetcode id=1338 lang=rust
 *
 * [1338] Reduce Array Size to The Half
 */

// @lc code=start
impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
       let mut m = std::collections::HashMap::new();
        for &a in &arr{
            *m.entry(a).or_insert(0)+=1;
        }
        let mut a :std::collections::BinaryHeap<i32>= m.iter().map(|x|*x.1).collect();
        let mut ans = 0;
        let mut cnt = 0;
        while let Some(i)=a.pop(){
            cnt +=i;
                        ans+=1;

            if cnt >=arr.len() as i32/2{
                return ans;
            }
        }
        ans
    }
}
// @lc code=end

