/*
 * @lc app=leetcode id=2157 lang=rust
 *
 * [2157] Groups of Strings
 */

// @lc code=start
impl Solution {
    pub fn group_strings(words: Vec<String>) -> Vec<i32> {
        let mut wordmasks = std::collections::HashMap::new();
        for word in &words {
            let mut mask = 0;
            for b in word.bytes() {
                mask |= (1 << (b - b'a') as u32);
            }
            *wordmasks.entry(mask).or_insert(0) += 1;
        }
        let get_adjacent = |mask: i32| {
            let mut adj = Vec::new();
            for i in 0..26 {
                adj.push(mask ^ (1 << i));
            }
            for i in 0..26 {
                if mask & (1 << i) == 0 {
                    continue;
                }
                for j in 0..26 {
                    if mask & (1 << j) == 0 {
                        adj.push(mask ^ (1 << i) ^ (1 << j));
                    }
                }
            }

            adj
        };
        let mut used = std::collections::HashSet::new();
        let mut best = 0;
        let mut cnt = 0;
        for (&mask, &occ) in &wordmasks {
            if used.contains(&mask) {
                continue;
            }
            let mut q = std::collections::VecDeque::from([mask]);
            used.insert(mask);
            let mut total = occ;
            while let Some(u) = q.pop_front() {
                for v in get_adjacent(u) {
                    if used.contains(&v) {
                        continue;
                    }
                    if let Some(&w) = wordmasks.get(&v) {
                        q.push_back(v);
                        used.insert(v);
                        total += w;
                    }
                }
            }
            best = best.max(total);
            cnt += 1;
        }
        vec![cnt, best]
    }
}
// @lc code=end
impl Solution {
    pub fn group_strings(words: Vec<String>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut parent=HashMap::new();
        let mut size=HashMap::new();
            let mut groups=words.len() as i32;
        let mut max_size=0;
        fn find(x:i32,parent:&mut HashMap<i32,i32>)->i32{
            let px=*parent.get(&x).unwrap_or(&-1);
            if px!=x{
                let fx=find(px,parent);
                parent.insert(x,fx);
            }
            parent[&x]
        }
        let unite=|x:i32,y:i32,parent:&mut HashMap<i32,i32>,size:&mut HashMap<i32,i32>,groups:&mut i32,max_size:&mut i32|{
            if !parent.contains_key(&y){
                return
            }
            let (px,py)=(find(x,parent),find(y,parent));
            if px!=py{
                parent.insert(px,py);
                *size.entry(py).or_insert(0)+=size[&px];
                *max_size=size[&py].max(*max_size);
                *groups-=1;
            }

        };
        for word in &words{
            let mut x=0;
            for b in word.bytes(){
                x|=(1<<(b-b'a'));
               
            }
             parent.insert(x,x);
                *size.entry(x).or_insert(0)+=1;
                max_size=size[&x].max(max_size);
                if size[&x]>1{
                    groups-=1;
                }
        }
        
        let p:Vec<i32>=parent.keys().cloned().collect();
        for &x in &p{
            for i in 0..26{
                unite(x,x^(1<<i),&mut parent,&mut size,&mut groups,&mut max_size);
                if ((x>>i)&1)>0{
                    for j in 0..26{
                        if ((x>>j)&1)==0{
                            unite(x,x^(1<<i)|(1<<j),&mut parent,&mut size,&mut groups,&mut max_size);
                        }
                    }
                }
            }
        }
        vec![groups,max_size]
    }
}