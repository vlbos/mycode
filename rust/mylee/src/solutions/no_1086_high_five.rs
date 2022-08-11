// 1086\. High Five
// ================

// Given a list of scores of different students, return the average score of each student's **top five scores** in **the order of each student's id**.

// Each entry `items[i]` has `items[i][0]` the student's id, and `items[i][1]` the student's score.  The average score is calculated using integer division.

// **Example 1:**

// **Input:** \[\[1,91\],\[1,92\],\[2,93\],\[2,97\],\[1,60\],\[2,77\],\[1,65\],\[1,87\],\[1,100\],\[2,100\],\[2,76\]\]
// **Output:** \[\[1,87\],\[2,88\]\]
// **Explanation:**
// The average of the student with id = 1 is 87.
// The average of the student with id = 2 is 88.6. But with integer division their average converts to 88.

// **Note:**

// 1.  `1 <= items.length <= 1000`
// 2.  `items[i].length == 2`
// 3.  The IDs of the students is between `1` to `1000`
// 4.  The score of the students is between `1` to `100`
// 5.  For each student, there are at least 5 scores

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Goldman Sachs](https://leetcode.ca/tags/#Goldman%20Sachs)
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cnt = std::collections::HashMap::<i32, (i32, i32)>::new();
        for item in &items {
            cnt.entry(item[0])
                .and_modify(|mut x| {
                    x.0 += item[1];
                    x.1 += 1;
                })
                .or_insert((item[1], 1));
        }
        let mut average_scores: Vec<(i32, i32)> =
            cnt.into_iter().map(|(k, v)| (-v.0 / v.1, k)).collect();
        average_scores.sort();
        (if average_scores.len() > 5 {
            &average_scores[..5]
        } else {
            &average_scores[..]
        })
        .iter()
        .map(|x| vec![x.1, -x.0])
        .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_high_five_1() {
        assert_eq!(
            vec![vec![1, 87], vec![2, 88]],
            Solution::high_five(vec![
                vec![1, 91],
                vec![1, 92],
                vec![2, 93],
                vec![2, 97],
                vec![1, 60],
                vec![2, 77],
                vec![1, 65],
                vec![1, 87],
                vec![1, 100],
                vec![2, 100],
                vec![2, 76]
            ])
        );
    }
}
