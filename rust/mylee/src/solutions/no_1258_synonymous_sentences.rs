// 1258\. Synonymous Sentences
// ===========================

// Given a list of pairs of equivalent words `synonyms` and a sentence `text`, Return all possible synonymous sentences **sorted lexicographically**.

// **Example 1:**

// **Input:** synonyms = \[\["happy","joy"\],\["sad","sorrow"\],\["joy","cheerful"\]\],
// text = "I am happy today but was sad yesterday"
// **Output:** \["I am cheerful today but was sad yesterday",
// ​​​​​​"I am cheerful today but was sorrow yesterday",
// "I am happy today but was sad yesterday",
// "I am happy today but was sorrow yesterday",
// "I am joy today but was sad yesterday",
// "I am joy today but was sorrow yesterday"\]

// **Constraints:**

// *   `0 <= synonyms.length <= 10`
// *   `synonyms[i].length == 2`
// *   `synonyms[0] != synonyms[1]`
// *   All words consist of at most `10` English letters only.
// *   `text` is a single space separated sentence of at most `10` words.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Cruise Automation](https://leetcode.ca/tags/#Cruise%20Automation)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn generate_sentences(synonyms: Vec<Vec<String>>, text: String) -> Vec<String> {
        let text: Vec<&str> = text.split_ascii_whitespace().collect();
        let mut q = std::collections::VecDeque::from([text.clone()]);
        for s in &synonyms {
            let mut i = 0;
            while i < q.len() {
                let text = q[i].clone();
                if let Some(j) = text.iter().position(|x| x == &s[0]) {
                    let mut new_text = text.clone();
                    new_text[j] = s[1].as_str();
                    if new_text != text && !q.contains(&new_text) {
                        q.push_back(new_text.clone());
                    }
                }
                if let Some(j) = text.iter().position(|x| x == &s[1]) {
                    let mut new_text = text.clone();
                    new_text[j] = s[0].as_str();
                    if new_text != text && !q.contains(&new_text) {
                        q.push_back(new_text.clone());
                    }
                }
                i += 1;
            }
        }
        let mut ans: Vec<String> = q.iter().map(|x| x.join(" ")).collect();
        ans.sort();
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_generate_sentences_1() {
        assert_eq!(
            [
                "I am cheerful today but was sad yesterday",
                "I am cheerful today but was sorrow yesterday",
                "I am happy today but was sad yesterday",
                "I am happy today but was sorrow yesterday",
                "I am joy today but was sad yesterday",
                "I am joy today but was sorrow yesterday"
            ]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>(),
            Solution::generate_sentences(
                vec![
                    vec!["happy".to_string(), "joy".to_string()],
                    vec!["sad".to_string(), "sorrow".to_string()],
                    vec!["joy".to_string(), "cheerful".to_string()]
                ],
                String::from("I am happy today but was sad yesterday")
            )
        );
    }
}
