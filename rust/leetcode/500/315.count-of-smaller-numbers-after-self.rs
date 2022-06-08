/*
 * @lc app=leetcode id=315 lang=rust
 *
 * [315] Count of Smaller Numbers After Self
 */

// @lc code=start
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut a = nums.clone();
        a.sort();
        a.dedup();
        let n = nums.len();
        let mut c = vec![0; n + 5];
        let low_bit = |x: i32| -> i32 { x & (-x) };
        let update = |mut pos: usize, c: &mut Vec<i32>| {
            while pos < c.len() {
                c[pos] += 1;
                pos += low_bit(pos as i32) as usize;
            }
        };
        let query = |mut pos: i32,c:&Vec<i32>| -> i32 {
            let mut ans = 0;
            while pos > 0 && pos <c.len() as i32{
                ans += c[pos as usize];
                pos -= low_bit(pos);
            }
            ans
        };
        let get_id = |x: i32,a:&Vec<i32>| -> i32 {
           ( match a.binary_search(&x) {
                Ok(i) => i + 1,
                Err(i) => i + 1,
            }) as _
        };
        for i in (0..n).rev() {
            let id = get_id(nums[i],&a);
            ans.push(query(id - 1,&c));
            update(id as usize,&mut c);
        }
        ans.reverse();
        ans
    }
}
// @lc code=end
