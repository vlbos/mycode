/*
 * @lc app=leetcode id=535 lang=rust
 *
 * [535] Encode and Decode TinyURL
 */

// @lc code=start
struct Codec {
    m:std::collections::HashMap<String,String>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
            Self{m:std::collections::HashMap::new()}
    }
	
    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {
        let a = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut ans = String::new();
        for _ in 0..6{
            ans.push(a[rng.gen_range(0,62) as usize]);
        }
        self.m.insert(ans.clone(),longURL);
       "http://tinyurl.com/".to_string()+ &ans
    }
	
    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        let mut s = shortURL;
        if let Some(ss)=self.m.get(&s.replace("http://tinyurl.com/","")){
        return (*ss).clone();
        }
        String::new()
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */
// @lc code=end

