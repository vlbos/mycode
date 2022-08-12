// 544\. Output Contest Matches
// ============================

// During the NBA playoffs, we always arrange the rather strong team to play with the rather weak team,
// like make the rank 1 team play with the rank nth team, which is a good strategy to make the contest more interesting.
// Now, you're given **n** teams, you need to output their **final** contest matches in the form of a string.

// The **n** teams are given in the form of positive integers from 1 to n, which represents their initial rank.
// (Rank 1 is the strongest team and Rank n is the weakest team.) We'll use parentheses('(', ')') and commas(',')
// to represent the contest team pairing - parentheses('(' , ')') for pairing and commas(',') for partition.
// During the pairing process in each round, you always need to follow the strategy of making the rather strong one pair with the rather weak one.

// **Example 1:**

// **Input:** 2
// **Output:** (1,2)
// **Explanation:**
// Initially, we have the team 1 and the team 2, placed like: 1,2.
// Then we pair the team (1,2) together with '(', ')' and ',', which is the final answer.

// **Example 2:**

// **Input:** 4
// **Output:** ((1,4),(2,3))
// **Explanation:**
// In the first round, we pair the team 1 and 4, the team 2 and 3 together, as we need to make the strong team and weak team together.
// And we got (1,4),(2,3).
// In the second round, the winners of (1,4) and (2,3) need to play again to generate the final winner, so you need to add the paratheses outside them.
// And we got the final answer ((1,4),(2,3)).

// **Example 3:**

// **Input:** 8
// **Output:** (((1,8),(4,5)),((2,7),(3,6)))
// **Explanation:**
// First round: (1,8),(2,7),(3,6),(4,5)
// Second round: ((1,8),(4,5)),((2,7),(3,6))
// Third round: (((1,8),(4,5)),((2,7),(3,6)))
// Since the third round will generate the final winner, you need to output the answer (((1,8),(4,5)),((2,7),(3,6))).

// **Note:**

// 1.  The **n** is in range \[2, 212\].
// 2.  We ensure that the input **n** can be converted into the form 2k, where k is a positive integer.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
// use std::collections::VecDeque;

impl Solution {
    pub fn   find_contest_match(n: i32) -> String {
        // let mut last_queue = (1..=n)
        //     .map(|c| c.to_string().chars().collect::<VecDeque<char>>())
        //     .collect::<VecDeque<_>>();
        // loop {
        //     let mut curr_queue = VecDeque::new();
        //     while !last_queue.is_empty() {
        //         let mut high = last_queue.pop_front().unwrap();
        //         let low = last_queue.pop_back().unwrap();
        //         high.push_back(',');
        //         high.extend(low.into_iter());
        //         high.push_back(')');
        //         high.push_front('(');
        //         curr_queue.push_back(high);
        //     }
        //     last_queue = curr_queue;
        //     if last_queue.len() <= 1 {
        //         break;
        //     }
        // }
        // last_queue
        //     .pop_front()
        //     .map(|s| s.into_iter().collect::<String>())
        //     .unwrap_or_else(String::new)
        let mut cur: Vec<String> = (1..n).map(|x| x.to_string()).collect();
        while cur.len() > 1 {
            let mut next = Vec::new();
            for i in 0..cur.len() / 2 {
                next.push(format!("({},{})", cur[i], cur[cur.len() - 1 - i]));
            }
            cur = next;
        }
        cur[0].clone()
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_output_contest_matches_1() {
        assert_eq!(Solution::find_contest_match(2), String::from("(1,2)"));
    }

    #[test]
   pub fn  test_output_contest_matches_2() {
        assert_eq!(
            Solution::find_contest_match(4),
            String::from("((1,4),(2,3))")
        );
    }

    #[test]
   pub fn  test_output_contest_matches_3() {
        assert_eq!(
            Solution::find_contest_match(8),
            String::from("(((1,8),(4,5)),((2,7),(3,6)))")
        );
    }
}
