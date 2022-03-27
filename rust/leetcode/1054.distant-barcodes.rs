/*
 * @lc app=leetcode id=1054 lang=rust
 *
 * [1054] Distant Barcodes
 */

// @lc code=start
impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();
        for &b in &barcodes{
            *m.entry(b).or_insert(0)+=1;
        }
        let mut mh = m.iter().map(|x|(*x.1,*x.0)).collect::<std::collections::BinaryHeap<(i32,i32)>>();
        let mut ans = vec![0;barcodes.len()];
        let mut i = 0;
        while let Some((cnt,num))=mh.pop(){
            for _ in 0..cnt{
                ans[i]= num;
                i+=2;
                if i>=ans.len(){
                i=1;
                }
            }
        }
        ans
    }
}
// @lc code=end

