294\. 计算重复

*    [题目](https://www.acwing.com/problem/content/description/296/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/296/1/)
*    [题解](https://www.acwing.com/problem/content/solution/296/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/296/)

  

定义 conn(s,n)conn(s,n) 为 nn 个字符串 ss 首尾相接形成的字符串，例如：

conn(“abc”,2)\=”abcabc”conn(“abc”,2)\=”abcabc”

称字符串 aa 能由字符串 bb 生成，当且仅当从字符串 bb 中删除某些字符后可以得到字符串 aa。

例如 `abdbec` 可以生成 `abc`，但是 `acbbe` 不能生成 `abc`。

给定两个字符串 s1s1 和 s2s2，以及两个整数 n1n1 和 n2n2，求一个最大的整数 mm，满足 conn(conn(s2,n2),m)conn(conn(s2,n2),m) 能由 conn(s1,n1)conn(s1,n1) 生成。

#### 输入格式

输入包含多组测试数据。

每组数据由 22 行组成，第一行包含 s2,n2s2,n2，第二行包含 s1,n1s1,n1。

#### 输出格式

对于每组数据输出一行表示答案 mm。

#### 数据范围

s1s1 和 s2s2 长度不超过 100100，n1n1 和 n2n2 不大于 106106。

#### 输入样例：

    ab 2
    acb 4
    acb 1
    acb 1
    aa 1
    aaa 3
    baab 1
    baba 11
    aaaaa 1
    aaa 20
    

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

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3845&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3845&show_algorithm_tags=1)[倍增优化DP](https://www.acwing.com/problem/search/1/?search_content=%E5%80%8D%E5%A2%9E%E4%BC%98%E5%8C%96DP&source_file_id=3845&show_algorithm_tags=1)