380\. 舞动的夜晚

*    [题目](https://www.acwing.com/problem/content/description/382/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/382/1/)
*    [题解](https://www.acwing.com/problem/content/solution/382/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/382/)

  

LL 公司和 HH 公司举办了一次联谊晚会。

晚会上，LL 公司的 NN 位员工和 HH 公司的 MM 位员工打算进行一场交际舞。

在这些领导中，一些 LL 公司的员工和 HH 公司的员工之间是互相认识的，这样的认识关系一共有 TT 对。

舞会上，每位员工会尝试选择一名 Ta 认识的对方公司的员工作为舞伴，并且每位员工至多跳一支舞。

完成的交际舞的数量越多，晚会的气氛就越热烈。

顾及到晚会的气氛，员工们希望知道，哪些员工之间如果进行了交际舞，就会使整场晚会能够完成的交际舞的最大数量减小。

#### 输入格式

第一行三个整数 N、M、TN、M、T。

接下来 TT 行每行两个整数 x、yx、y，表示 LL 公司的员工 xx 和 HH 公司的员工 yy 互相认识。

#### 输出格式

第一行一个整数 cntcnt，表示进行了交际舞后会使整场晚会能够完成的交际舞的最大数量减小的员工有多少对。

第二行 cntcnt 个整数，升序输出这样的一对员工的认识关系的编号（他们的认识关系是在输入数据中读入的第几条认识关系）。

如果 cnt\=0cnt\=0，输出一个空行。

#### 数据范围

1≤N,M≤100001≤N,M≤10000,  
1≤T≤1000001≤T≤100000,  
1≤x≤N1≤x≤N,  
1≤y≤M1≤y≤M

#### 输入样例：

    3 3 6
    1 1
    2 1
    2 2
    3 1
    3 2
    3 3
    

#### 输出样例：

    3
    2 4 5
    

难度：困难

时/空限制：1s / 64MB

总通过数：640

总尝试数：1986

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3931&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3931&show_algorithm_tags=1)[最大流](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E5%A4%A7%E6%B5%81&source_file_id=3931&show_algorithm_tags=1)