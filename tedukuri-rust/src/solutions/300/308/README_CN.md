308\. 它们中的多少个

*    [题目](https://www.acwing.com/problem/content/description/310/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/310/1/)
*    [题解](https://www.acwing.com/problem/content/solution/310/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/310/)

  

在无向连通图中，若一条边被删除后，图会分成不连通的两部分，则称该边为割边。

求满足如下条件的无向连通图的数量：

1、由 NN 个节点构成，节点有标号，编号为 1∼N1∼N。

2、割边**不超过** MM 条。

3、没有自环和重边。

#### 注意

本题遵循书中所述，割边限制为不超过 MM 条，输入输出也与题目描述保持一致。

与 [Conster Hunter](http://noi-test.zzstep.com/contest/0x50%E3%80%8C%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92%E3%80%8D%E4%BE%8B%E9%A2%98/5C01%20How%20Many%20of%20Them%3F) 网站关于本题的描述和要求略有不同。

#### 输入格式

输入共一行，包含两个整数 NN 和 MM。

#### 输出格式

输出一个整数表示满足条件的无相连通图的数量对 109+7109+7 取模后的结果。

#### 数据范围

2≤N≤502≤N≤50,  
0≤M≤N∗(N−1)/20≤M≤N∗(N−1)/2

#### 输入样例：

    3 3
    

#### 输出样例：

    4
    

难度：困难

时/空限制：1s / 64MB

总通过数：350

总尝试数：872

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3859&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3859&show_algorithm_tags=1)[计数类DP](https://www.acwing.com/problem/search/1/?search_content=%E8%AE%A1%E6%95%B0%E7%B1%BBDP&source_file_id=3859&show_algorithm_tags=1)