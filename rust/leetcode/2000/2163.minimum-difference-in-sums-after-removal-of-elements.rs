/*
 * @lc app=leetcode id=2163 lang=rust
 *
 * [2163] Minimum Difference in Sums After Removal of Elements
 */

// @lc code=start
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n = nums.len() / 3;
        let mut part1 = vec![0; n + 1];
        let mut sum = nums[..n].iter().map(|x| *x as i64).sum::<i64>();
        use std::collections::BinaryHeap;
        let mut q1 = BinaryHeap::from(nums[..n].to_vec());
        part1[0] = sum;
        for (i,&num) in nums[n..n * 2].iter().enumerate() {
            sum += num as i64;
            q1.push(num);
            sum -= q1.pop().unwrap() as i64;
            part1[i+1] = sum;
        }
        let mut part2 = nums[n * 2..n * 3].iter().map(|x| *x as i64).sum::<i64>();
        let mut q2 = BinaryHeap::from(nums[n * 2..n * 3].iter().map(|x| -*x).collect::<Vec<i32>>());
        let mut ans = part1[n] - part2;
        for (i, &num) in nums[n..n * 2].iter().enumerate().rev() {
            part2 += num as i64;
            q2.push(-num);
            part2 += q2.pop().unwrap() as i64;
            ans = ans.min(part1[i] - part2);
        }
        ans
    }
}
// @lc code=end
