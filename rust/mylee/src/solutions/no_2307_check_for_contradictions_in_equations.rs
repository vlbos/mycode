// # [2307. Check for Contradictions in Equations](https://leetcode.com/problems/check-for-contradictions-in-equations)

// ## Description

// You are given a 2D array of strings equations and an array of real numbers values,
// where equations[i] = [Ai, Bi] and values[i] means that Ai / Bi = values[i].

// Determine if there exists a contradiction in the equations. Return true if there is a contradiction, or false otherwise.

// Note:

//
// 	When checking if two numbers are equal, check that their absolute difference is less than 10-5.
// 	The testcases are generated such that there are no cases targeting precision, i.e. using double is enough to solve the problem.
//

// Example 1:

//
// Input: equations = [["a","b"],["b","c"],["a","c"]], values = [3,0.5,1.5]
// Output: false
// Explanation:
// The given equations are: a / b = 3, b / c = 0.5, a / c = 1.5
// There are no contradictions in the equations. One possible assignment to satisfy all equations is:
// a = 3, b = 1 and c = 2.
//

// Example 2:

//
// Input: equations = [["le","et"],["le","code"],["code","et"]], values = [2,5,0.5]
// Output: true
// Explanation:
// The given equations are: le / et = 2, le / code = 5, code / et = 0.5
// Based on the first two equations, we get code / et = 0.4.
// Since the third equation is code / et = 0.5, we get a contradiction.
//

// Constraints:

//
// 	1 <= equations.length <= 100
// 	equations[i].length == 2
// 	1 <= Ai.length, Bi.length <= 5
// 	Ai, Bi consist of lowercase English letters.
// 	equations.length == values.length
// 	0.0 < values[i] <= 10.0
// 	values[i] has a maximum of 2 decimal places.
//

//   bool check_contradictions(vector<vector<string>>& equations,
//                            vector<double>& values) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn check_contradictions(equations: Vec<Vec<String>>, values: Vec<f64>) -> bool {
        let mut m = std::collections::HashMap::new();
        for e in &equations {
            for u in e {
                let len = m.len();
                m.entry(u).or_insert(len);
            }
        }
        let mut g = vec![Vec::new(); m.len()];
        let mut seen = vec![0.0; g.len()];
        for (i, e) in equations.iter().enumerate() {
            let (u, v) = (
                *m.get(&e[0]).unwrap() as usize,
                *m.get(&e[1]).unwrap() as usize,
            );
            g[u].push((v, values[i]));
            g[v].push((u, 1.0 / values[i]));
        }
        fn dfs(u: usize, val: f64, g: &Vec<Vec<(usize, f64)>>, seen: &mut Vec<f64>) -> bool {
            if seen[u] > 0.00001 {
                return (val / seen[u] - 1.0).abs() > 0.00001;
            }
            seen[u] = val;
            for &(v, w) in &g[u] {
                if dfs(v, val / w, g, seen) {
                    return true;
                }
            }
            false
        }
        for i in 0..g.len() {
            if seen[i] < 0.00001 && dfs(i, 1.0, &g, &mut seen) {
                return true;
            }
        }
        false
    }
}

// impl Solution {
//     pub fn check_contradictions(equations: Vec<Vec<String>>, values: Vec<f64>) -> bool {
//             use std::collections::HashMap;
//  let mut m = HashMap::new();
//         let mut g = HashMap::new();
//         for (i, e) in equations.iter().enumerate() {
//             let (u, v) = (
//                 e[0].clone() ,
//                e[1].clone(),
//             );
//             g.entry(u.clone()).or_insert(Vec::new()).push((v.clone(), values[i]));
//             g.entry(v.clone()).or_insert(Vec::new()).push((u.clone(), 1.0 / values[i]));
//         }
//         fn dfs(u: &String, val: f64, g: &HashMap<String,Vec<(String, f64)>>, m: &mut HashMap<String,f64>,ans:&mut bool){
//             if *ans{
//                 return 
//             }
//            if let Some(&v)=m.get(u) {
//                 if (v-val).abs()>1e-5{
//                     *ans=true;
//                 }
//                 return
//            }
//            m.insert(u.clone(),val);

//             for (v, w) in g.get(u).unwrap_or(&Vec::new()) {
//                 dfs(v, val / w, g, m,ans);
//                 }
            
//         }
//         let mut ans=false;
//         for e in &equations {
//             if !m.contains_key(&e[0]) {
//                  dfs(&e[0], 1.0, &g, &mut m,&mut ans) 
//             }
           
//         }
//         ans
//     }
// }


// class Solution:
//     def checkContradictions(self, equations: List[List[str]], values: List[float]) -> bool:
//         edges = defaultdict(set)
//         for j,(x,y) in enumerate(equations):
//             edges[x].add((y,values[j])) 
//             edges[y].add((x,1/values[j]))  #图中的边一定要成对添加
//         val = {}
//         ans = False #初始默认没有矛盾

//         def dfs(node,v):
//             nonlocal ans
//             if ans: #已经有矛盾就不浪费时间了
//                 return
//             if node in val:
//                 if abs(val[node]-v)>1e-5: #题目明确说不超过1e-5视为相同值
//                     ans=True
//                 return #搜过的点，不管有没有矛盾，都应直接return，否则程序就会死循环         
//             val[node]=v
//             for nei,rat in edges[node]:
//                 dfs(nei,v/rat)
        
//         for x,y in equations:
//             if x not in val: # 如果x还没有值，y也一定没有，这时说明遇到了一个新的连通分量
//                 dfs(x,1.)
//         return ans


#[cfg(test)]
mod test {
    use super::*;
// [["qnu","dravf"],["dravf","f"],["f","xv"],["xv","p"],["p","ybp"],["ybp","nqyj"],["nqyj","t"],["t","elg"],["elg","unpca"],["unpca","qma"],["qma","ugznq"],["ugznq","siwp"],["siwp","i"],["i","ht"],["ht","pmb"],["pmb","nkc"],["nkc","jrr"],["jrr","olmbs"],["olmbs","v"],["v","o"],["o","mi"],["mi","hizj"],["hizj","bmx"],["bmx","g"],["g","ni"],["ni","y"],["y","hzg"],["hzg","xlq"],["xlq","u"],["u","r"],["r","ay"],["ay","mt"],["mt","s"],["s","ef"],["ef","bea"],["bea","fs"],["fs","hkrv"],["hkrv","sd"],["sd","d"],["d","w"],["w","piczb"],["piczb","zl"],["zl","pcjux"],["pcjux","a"],["a","dbn"],["dbn","gvcad"],["gvcad","gzvcy"],["gzvcy","hjym"],["hjym","mhmnt"],["mhmnt","ewc"],["ewc","sp"],["sp","hqxao"],["hqxao","wypzn"],["wypzn","gvymw"],["gvymw","mmvda"],["mmvda","jlle"],["jlle","pgekr"],["pgekr","nqq"],["nqq","gyj"],["gyj","fa"],["fa","tzlw"],["tzlw","rv"],["rv","ohqpp"],["ohqpp","lfwz"],["lfwz","xseog"],["xseog","teb"],["teb","xiaqw"],["xiaqw","mapgi"],["mapgi","zm"],["zm","ww"],["ww","wpfen"],["wpfen","rrz"],["rrz","or"],["or","pi"],["pi","vn"],["vn","k"],["k","h"],["h","xvy"],["xvy","wame"],["wame","xw"],["xw","amxsi"],["amxsi","nhqg"],["nhqg","rgm"],["rgm","rmyu"],["rmyu","qmtc"],["qmtc","sib"],["sib","zruf"],["zruf","najh"],["najh","x"],["x","pw"],["pw","oqfg"],["oqfg","l"],["l","catd"],["catd","ax"],["ax","xk"],["xk","fhabv"],["fhabv","lvjx"],["lvjx","vuugv"],["vuugv","b"],["b","uzsx"]]
// [3.27,8.68,5.75,6.61,1.29,6.85,9.43,4.36,1.66,3.1,2.02,5.66,2.15,1.96,4.39,0.6,2.08,1.58,8.2,7.3,5.71,7.65,8.96,5.18,3.11,7.82,9.76,0.46,4.9,6.55,9.39,5.03,7.31,4.79,8.64,4.94,8.95,0.77,8.79,5.33,4.06,5.34,6.77,0.01,4.96,1.04,7.7,2.18,1.33,8.36,6.59,4.42,6.0,3.81,0.75,0.3,0.67,3.75,3.08,3.55,8.34,5.31,8.94,9.77,0.38,2.56,0.61,4.78,8.68,8.49,2.04,5.19,9.19,7.22,5.19,8.12,6.81,3.81,9.64,4.54,9.04,1.25,1.26,5.49,4.08,8.82,1.38,3.56,4.26,1.61,1.78,7.02,9.61,1.63,5.97,7.35,3.12,2.21,0.03,2.9]

// Expected: false
// <!---->


// [["zzy","zzz"],["zzz","aaa"],["aaa","aab"],["aab","aac"],["aac","aad"],["aad","aae"],["aaa","aag"],["aag","aah"],["aah","aai"],["aai","aae"]]
// [0.01,0.01,2.00,2.01,2.03,2.06,1.99,2.02,2.04,2.05]
// Expected: true
// <!---->


// [["aaa","aac"],["aac","aad"],["aad","aae"],["aae","aaf"],["aaf","aag"],["aag","aah"],["aah","aai"],["aai","aaj"],["aaj","aak"],["aak","aal"],["aal","aam"],["aam","aan"],["aan","aao"],["aao","aap"],["aap","aaq"],["aaq","aar"],["aar","aas"],["aas","aat"],["aat","aau"],["aau","aav"],["aav","aaw"],["aaw","aax"],["aax","aay"],["aay","aaz"],["aaz","aba"],["aba","abb"],["abb","abc"],["abc","abd"],["abd","abe"],["abe","abf"],["abf","abg"],["abg","abh"],["abh","abi"],["abi","abj"],["abj","abk"],["abk","abl"],["abl","abm"],["abm","abn"],["abn","abo"],["abo","abp"],["abp","abq"],["abq","abr"],["abr","abs"],["abs","abt"],["abt","abu"],["abu","abv"],["abv","abw"],["aaa","aby"],["aby","abz"],["abz","aca"],["aca","acb"],["acb","acc"],["acc","acd"],["acd","ace"],["ace","acf"],["acf","acg"],["acg","ach"],["ach","aci"],["aci","acj"],["acj","ack"],["ack","acl"],["acl","acm"],["acm","acn"],["acn","aco"],["aco","acp"],["acp","acq"],["acq","acr"],["acr","acs"],["acs","act"],["act","acu"],["acu","acv"],["acv","acw"],["acw","acx"],["acx","acy"],["acy","acz"],["acz","ada"],["ada","adb"],["adb","adc"],["adc","add"],["add","ade"],["ade","adf"],["adf","adg"],["adg","adh"],["adh","adi"],["adi","adj"],["adj","adk"],["adk","adl"],["adl","adm"],["adm","adn"],["adn","ado"],["ado","adp"],["adp","adq"],["adq","adr"],["adr","ads"],["ads","abw"]]
// [1.03,1.05,1.06,1.08,1.11,1.13,1.15,1.16,1.19,1.20,1.23,1.25,1.26,1.28,1.30,1.32,1.34,1.37,1.39,1.41,1.42,1.45,1.46,1.48,1.50,1.52,1.55,1.56,1.58,1.61,1.62,1.65,1.67,1.69,1.71,1.72,1.75,1.77,1.79,1.81,1.82,1.84,1.86,1.89,1.90,1.93,1.94,1.01,1.02,1.04,1.07,1.09,1.10,1.12,1.14,1.17,1.18,1.21,1.22,1.24,1.27,1.29,1.31,1.33,1.35,1.36,1.38,1.40,1.43,1.44,1.47,1.49,1.51,1.53,1.54,1.57,1.59,1.60,1.63,1.64,1.66,1.68,1.70,1.73,1.74,1.76,1.78,1.80,1.83,1.85,1.87,1.88,1.91,1.92,1.95]
// Expected: false
// <!---->


// [["zzz","aaa"],["aaa","aac"],["aac","aad"],["aad","aae"],["aae","aaf"],["aaf","aag"],["aag","aah"],["aah","aai"],["aai","aaj"],["aaj","aak"],["aak","aal"],["aal","aam"],["aam","aan"],["aan","aao"],["aao","aap"],["aap","aaq"],["aaq","aar"],["aar","aas"],["aas","aat"],["aat","aau"],["aau","aav"],["aav","aaw"],["aaw","aax"],["aax","aay"],["aay","aaz"],["aaz","aba"],["aba","abb"],["abb","abc"],["abc","abd"],["abd","abe"],["abe","abf"],["abf","abg"],["abg","abh"],["abh","abi"],["abi","abj"],["abj","abk"],["abk","abl"],["abl","abm"],["abm","abn"],["abn","abo"],["abo","abp"],["abp","abq"],["abq","abr"],["abr","abs"],["abs","abt"],["abt","abu"],["abu","abv"],["abv","abw"],["zaa","aby"],["aby","abz"],["abz","aca"],["aca","acb"],["acb","acc"],["acc","acd"],["acd","ace"],["ace","acf"],["acf","acg"],["acg","ach"],["ach","aci"],["aci","acj"],["acj","ack"],["ack","acl"],["acl","acm"],["acm","acn"],["acn","aco"],["aco","acp"],["acp","acq"],["acq","acr"],["acr","acs"],["acs","act"],["act","acu"],["acu","acv"],["acv","acw"],["acw","acx"],["acx","acy"],["acy","acz"],["acz","ada"],["ada","adb"],["adb","adc"],["adc","add"],["add","ade"],["ade","adf"],["adf","adg"],["adg","adh"],["adh","adi"],["adi","adj"],["adj","adk"],["adk","adl"],["adl","adm"],["adm","adn"],["adn","ado"],["ado","adp"],["adp","adq"],["adq","adr"],["adr","ads"],["ads","abw"],["zzz","zaa"]]
// [1.00,1.03,1.05,1.08,1.11,1.13,1.15,1.16,1.19,1.20,1.46,1.23,1.25,1.26,1.28,1.30,1.32,1.34,1.37,1.39,1.06,1.41,1.42,1.45,1.48,1.50,1.52,1.55,1.56,1.58,1.61,1.62,1.65,1.67,1.69,1.71,1.72,1.75,1.77,1.79,1.81,1.82,1.84,1.86,1.89,1.90,1.93,1.94,1.01,1.02,1.04,1.07,1.09,1.10,1.12,1.14,1.17,1.18,1.21,1.22,1.24,1.27,1.29,1.31,1.33,1.35,1.36,1.38,1.40,1.43,1.44,1.47,1.49,1.51,1.53,1.54,1.57,1.59,1.60,1.63,1.64,1.66,1.76,1.70,1.73,1.78,1.80,1.68,1.74,1.83,1.85,1.87,1.88,1.91,1.92,1.95,1.00]
// Expected: true
// <!---->


// [["aah","aai"],["acv","acw"],["aab","aac"],["aaw","aax"],["aav","aaw"],["abm","abn"],["adu","zza"],["aak","aal"],["acb","acc"],["adq","adr"],["aaq","aar"],["abq","abr"],["aan","aao"],["abe","abf"],["zzz","aca"],["abh","abi"],["ack","acl"],["abs","abt"],["adn","ado"],["aas","aat"],["aac","aad"],["abg","abh"],["abk","abl"],["adl","adm"],["abr","abs"],["abj","abk"],["abv","abw"],["acj","ack"],["acq","acr"],["add","ade"],["aag","aah"],["abu","abv"],["acs","act"],["aci","acj"],["acd","ace"],["aat","aau"],["act","acu"],["aad","aae"],["aaf","aag"],["ach","aci"],["aaa","aab"],["adj","adk"],["adr","ads"],["zza","zzb"],["abt","abu"],["aao","aap"],["adk","adl"],["aay","aaz"],["adb","adc"],["acy","acz"],["aco","acp"],["adt","adu"],["abc","abd"],["adh","adi"],["aca","acb"],["acm","acn"],["acf","acg"],["acz","ada"],["abi","abj"],["acx","acy"],["ace","acf"],["aal","aam"],["abd","abe"],["acp","acq"],["ada","adb"],["aar","aas"],["aaz","aba"],["abf","abg"],["adi","adj"],["zzz","aaa"],["abo","abp"],["adp","adq"],["ade","adf"],["ado","adp"],["abn","abo"],["acn","aco"],["adf","adg"],["adm","adn"],["acr","acs"],["abl","abm"],["abb","abc"],["aaj","aak"],["aba","abb"],["aam","aan"],["abw","abx"],["acg","ach"],["adg","adh"],["aax","aay"],["acc","acd"],["ads","adt"],["aai","aaj"],["aap","aaq"],["adc","add"],["aau","aav"],["acu","acv"],["acl","acm"],["acw","acx"],["aae","aaf"],["abp","abq"],["zzb","abx"]]
// [9.86,9.54,9.98,9.56,9.59,9.24,9.04,9.8,9.95,9.13,9.69,9.16,9.74,9.41,9.99,9.34,9.77,9.12,9.18,9.65,9.96,9.37,9.29,9.22,9.15,9.3,9.06,9.79,9.64,9.38,9.89,9.08,9.61,9.81,9.91,9.63,9.58,9.94,9.9,9.82,10.0,9.27,9.1,10.0,9.11,9.73,9.25,9.53,9.43,9.48,9.68,9.07,9.44,9.31,9.97,9.72,9.87,9.46,9.32,9.51,9.88,9.78,9.42,9.66,9.45,9.67,9.5,9.39,9.28,1.0,9.2,9.14,9.36,9.17,9.23,9.7,9.35,9.21,9.62,9.26,9.47,9.83,9.49,9.76,9.05,9.84,9.33,9.55,9.92,9.09,9.85,9.71,9.4,9.6,9.57,9.75,9.52,9.93,9.19,1.0]
// Expected: true
// <!---->


// [["abs","abt"],["abw","abx"],["aac","aad"],["acm","acn"],["abj","abk"],["aap","aaq"],["ade","adf"],["ads","adt"],["acl","acm"],["ada","adb"],["aan","aao"],["adg","adh"],["adt","adu"],["abd","abe"],["acr","acs"],["abi","abj"],["abc","abd"],["abq","abr"],["adh","adi"],["add","ade"],["aci","acj"],["adk","adl"],["abl","abm"],["aay","aaz"],["acw","acx"],["aab","aac"],["aaz","aba"],["act","acu"],["ado","adp"],["ace","acf"],["adu","zza"],["abv","abw"],["adn","ado"],["aas","aat"],["acd","ace"],["aao","aap"],["ack","acl"],["aaw","aax"],["acg","ach"],["aav","aaw"],["adp","adq"],["aaa","aab"],["zza","zzb"],["adm","adn"],["aaf","aag"],["abk","abl"],["aax","aay"],["abn","abo"],["acb","acc"],["adb","adc"],["acs","act"],["abg","abh"],["aar","aas"],["abf","abg"],["abe","abf"],["aak","aal"],["aaj","aak"],["adl","adm"],["aal","aam"],["acy","acz"],["ach","aci"],["abp","abq"],["aco","acp"],["zzz","aca"],["acz","ada"],["aaq","aar"],["abt","abu"],["abm","abn"],["acc","acd"],["zzz","aaa"],["aad","aae"],["acv","acw"],["abb","abc"],["acq","acr"],["abh","abi"],["aat","aau"],["acn","aco"],["adq","adr"],["abr","abs"],["adc","add"],["aag","aah"],["adj","adk"],["aba","abb"],["aau","aav"],["adi","adj"],["aca","acb"],["aae","aaf"],["aah","aai"],["acf","acg"],["aam","aan"],["abu","abv"],["acp","acq"],["acj","ack"],["abo","abp"],["adf","adg"],["acx","acy"],["adr","ads"],["aai","aaj"],["acu","acv"],["zzb","abx"]]
// [9.12,9.05,9.96,9.72,9.3,9.71,9.36,9.09,9.75,9.45,9.74,9.33,9.07,9.42,9.62,9.32,9.44,9.16,9.31,9.38,9.81,9.25,9.26,9.53,9.52,9.98,9.5,9.58,9.17,9.88,9.04,9.06,9.18,9.65,9.91,9.73,9.77,9.56,9.84,9.59,9.14,10.0,10.0,9.21,9.9,9.29,9.55,9.23,9.95,9.43,9.61,9.37,9.67,9.39,9.41,9.8,9.83,9.22,9.78,9.48,9.82,9.19,9.68,9.99,9.46,9.69,9.11,9.24,9.92,1.0,9.94,9.54,9.47,9.64,9.34,9.63,9.7,9.13,9.15,9.4,9.89,9.27,9.49,9.6,9.28,9.97,9.93,9.86,9.87,9.76,9.08,9.66,9.79,9.2,9.35,9.51,9.1,9.85,9.57,1.0]
   
 #[test]
    pub fn test_check_contradictions_1() {
        assert!(!Solution::check_contradictions(
            [["a", "b"], ["b", "c"], ["a", "c"]]
                .into_iter()
                .map(|x| x.into_iter().map(String::from).collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>(),
            vec![3.0, 0.5, 1.5]
        ));
    }
    #[test]
    pub fn test_check_contradictions_2() {
        assert!(Solution::check_contradictions(
            [["le", "et"], ["le", "code"], ["code", "et"]]
                .into_iter()
                .map(|x| x.into_iter().map(String::from).collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>(),
            vec![2.0, 5.0, 0.5]
        ));
    }
}
