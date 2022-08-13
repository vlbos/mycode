// 759\. Employee Free Time
// ========================

// We are given a list `schedule` of employees, which represents the working time for each employee.

// Each employee has a list of non-overlapping `Intervals`, and these intervals are in sorted order.

// Return the list of finite intervals representing **common, positive-length free time** for _all_ employees, also in sorted order.

// **Example 1:**

// **Input:** schedule = \[\[\[1,2\],\[5,6\]\],\[\[1,3\]\],\[\[4,10\]\]\]
// **Output:** \[\[3,4\]\]
// **Explanation:**
// There are a total of three employees, and all common
// free time intervals would be \[-inf, 1\], \[3, 4\], \[10, inf\].
// We discard any intervals that contain inf as they aren't finite.

// **Example 2:**

// **Input:** schedule = \[\[\[1,3\],\[6,7\]\],\[\[2,4\]\],\[\[2,5\],\[9,12\]\]\]
// **Output:** \[\[5,6\],\[7,9\]\]

// (Even though we are representing `Intervals` in the form `[x, y]`, the objects inside are `Intervals`,
// not lists or arrays. For example, `schedule[0][0].start = 1, schedule[0][0].end = 2`, and `schedule[0][0][0]` is not defined.)

// Also, we wouldn't include intervals like \[5, 5\] in our answer, as they have zero length.

// **Note:**

// 1.  `schedule` and `schedule[i]` are lists with lengths in range `[1, 50]`.
// 2.  `0 <= schedule[i].start < schedule[i].end <= 10^8`.

// **NOTE:** input types have been changed on June 17, 2019. Please reset to default code definition to get new method signature.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Airbnb](https://leetcode.ca/tags/#Airbnb) [Amazon](https://leetcode.ca/tags/#Amazon) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [DoorDash](https://leetcode.ca/tags/#DoorDash) [Google](https://leetcode.ca/tags/#Google) [Intuit](https://leetcode.ca/tags/#Intuit) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Pinterest](https://leetcode.ca/tags/#Pinterest) [Uber](https://leetcode.ca/tags/#Uber) [Wayfair](https://leetcode.ca/tags/#Wayfair)
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn employee_free_time(schedule: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>> {
        //    let mut schedule:Vec<Vec<i32>> =schedule.into_iter().flatten().collect();
        //     let mut ans=Vec::new();
        //     schedule.sort_by_key(|x|x[0]);
        //     let mut t= schedule[0].clone();
        //     for s in &schedule{
        //         if t[1]<s[0]{
        //         ans.push(vec![t[1],s[0]]);
        //         }
        //         if t[1]<s[1]{
        //            t=s.clone();
        //         }
        //     }
        //     ans
        let mut ans = Vec::new();
        let mut m = std::collections::BTreeMap::new();
        for employee in &schedule {
            for interval in employee {
                *m.entry(interval[0]).or_insert(0) += 1;
                *m.entry(interval[1]).or_insert(0) -= 1;
            }
        }
        let mut cnt = 0;
        for (&k, &v) in &m {
            cnt += v;
            if cnt == 0 {
                ans.push(vec![k, 0]);
            }
            if cnt > 0 && !ans.is_empty() && ans[ans.len() - 1][1] == 0 {
                ans.last_mut().unwrap()[1] = k;
            }
        }
        if !ans.is_empty() {
            ans.pop();
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_employee_free_time_1() {
        assert_eq!(
            vec![vec![3, 4]],
            Solution::employee_free_time(vec![
                vec![vec![1, 2], vec![5, 6]],
                vec![vec![1, 3]],
                vec![vec![4, 10]]
            ])
        );
    }
    #[test]
    pub fn test_employee_free_time_2() {
        assert_eq!(
            vec![vec![5, 6], vec![7, 9]],
            Solution::employee_free_time(vec![
                vec![vec![1, 3], vec![6, 7]],
                vec![vec![2, 4]],
                vec![vec![2, 5], vec![9, 12]]
            ])
        );
    }
}
