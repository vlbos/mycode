/*
 * @lc app=leetcode id=1600 lang=rust
 *
 * [1600] Throne Inheritance
 */

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;
struct ThroneInheritance {
   kingName:String,
   children:HashMap<String,Vec<String>>,
   dead:HashSet<String>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ThroneInheritance {

    fn new(kingName: String) -> Self {
        Self{kingName,children:HashMap::new(),dead:HashSet::new()}
    }
    
    fn birth(&mut self, parent_name: String, child_name: String) {
        self.children.entry(parent_name.clone()).or_default().push(child_name.clone());
    }
    
    fn death(&mut self, name: String) {
        self.dead.insert(name.clone());
    }
    
    fn get_inheritance_order(&self) -> Vec<String> {
        let mut ans = Vec::new();
       fn pre_order(dead:&HashSet<String>,children:&HashMap<String,Vec<String>>,p:&String,ans:&mut Vec<String>){
            if !dead.contains(p){
                ans.push(p.clone());
            }
            if let Some(cc)=children.get(p){
                 for c in cc{
                       pre_order(dead,children,c,ans);
                 }
           }
       }
        pre_order(&self.dead,&self.children,&self.kingName,&mut ans);
        ans
    }
}

/**
 * Your ThroneInheritance object will be instantiated and called as such:
 * let obj = ThroneInheritance::new(kingName);
 * obj.birth(parentName, childName);
 * obj.death(name);
 * let ret_3: Vec<String> = obj.get_inheritance_order();
 */
// @lc code=end

