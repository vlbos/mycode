// 1136\. Parallel Courses
// =======================

// There are `N` courses, labelled from 1 to `N`.

// We are given `relations[i] = [X, Y]`, representing a prerequisite relationship between course `X` and course `Y`: course `X` has to be studied before course `Y`.

// In one semester you can study any number of courses as long as you have studied all the prerequisites for the course you are studying.

// Return the minimum number of semesters needed to study all courses.  If there is no way to study all the courses, return `-1`.

// **Example 1:**

// **![](https://leetcode.ca/all/img/1136_1.png)**

// **Input:** N = 3, relations = \[\[1,3\],\[2,3\]\]
// **Output:** 2
// **Explanation:**
// In the first semester, courses 1 and 2 are studied. In the second semester, course 3 is studied.

// **Example 2:**

// **![](https://leetcode.ca/all/img/1136_2.png)**

// **Input:** N = 3, relations = \[\[1,2\],\[2,3\],\[3,1\]\]
// **Output:** \-1
// **Explanation:**
// No course can be studied because they depend on each other.

// **Note:**

// 1.  `1 <= N <= 5000`
// 2.  `1 <= relations.length <= 5000`
// 3.  `relations[i][0] != relations[i][1]`
// 4.  There are no repeated relations in the input.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google) [Uber](https://leetcode.ca/tags/#Uber)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut g = HashMap::new();
        let mut indegrees = HashMap::new();
        for r in &relations {
            g.entry(r[0]).or_insert(Vec::new()).push(r[1]);
            *indegrees.entry(r[1]).or_insert(0) += 1;
        }
        let mut q = std::collections::VecDeque::from(
            (1..=n)
                .filter(|x| !indegrees.contains_key(&x))
                .collect::<Vec<i32>>(),
        );
        let mut courses = 0;
        let mut semester = 0;
        while !q.is_empty() {
            let len = q.len() as i32;
            courses += len;
            for _ in 0..len {
                let cur = q.pop_front().unwrap();
                for &neighbor in g.get(&cur).unwrap_or(&Vec::new()) {
                    *indegrees.entry(neighbor).or_insert(0) -= 1;
                    if *indegrees.get(&neighbor).unwrap_or(&0) == 0 {
                        q.push_back(neighbor);
                    }
                }
            }
            semester += 1;
        }
        if courses == n {
            semester
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minimum_semesters_1() {
        assert_eq!(
            2,
            Solution::minimum_semesters(3, vec![vec![1, 3], vec![2, 3]])
        );
    }
    #[test]
    pub fn test_minimum_semesters_2() {
        assert_eq!(
            -1,
            Solution::minimum_semesters(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]])
        );
    }
}
