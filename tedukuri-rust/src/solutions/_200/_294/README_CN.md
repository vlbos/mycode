294\. 计算重复





 




  

定义 conn(s,n)conn(s,n) 为n个字符串s首尾相接形成的字符串，例如：

conn(“abc”,2)\=”abcabc”conn(“abc”,2)\=”abcabc”

称字符串a能由字符串b生成，当且仅当从字符串b中删除某些字符后可以得到字符串a。

例如 `abdbec` 可以生成 `abc`，但是 `acbbe` 不能生成 `abc`。

给定两个字符串s1和s2，以及两个整数n1和n2，求一个最大的整数m，满足 conn(conn(s2,n2),m)conn(conn(s2,n2),m) 能由 conn(s1,n1)conn(s1,n1) 生成。

#### 输入格式

输入包含多组测试数据。

每组数据由2行组成，第一行包含 s2,n2s2,n2，第二行包含 s1,n1s1,n1。

#### 输出格式

对于每组数据输出一行表示答案m。

#### 数据范围

s1s1 和s2长度不超过100，n1n1 和n2不大于106。

#### 输入样例：

    ab 2
    acb 4
    acb 1
    acb 1
   a1
   aa 3
    baab 1
   ba11
   aaa 1
   aa 20
    

#### 输出样例：

    2
    1
    4
    7
    12
    

难度：困难

时/空限制：1s / 64MB

总通过数：1124

总尝试数：2503

来源：

例题/0x50 动态规划/0x57 倍增优化DP/Count The Repetitions

算法标签

* 动态规划 
* 倍增优化DP 
