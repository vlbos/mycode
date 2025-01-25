349\. 黑暗城堡

*    [题目](https://www.acwing.com/problem/content/description/351/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/351/1/)
*    [题解](https://www.acwing.com/problem/content/solution/351/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/351/)

  

在顺利攻破 Lord lsp 的防线之后，lqr 一行人来到了 Lord lsp 的城堡下方。

Lord lsp 黑化之后虽然拥有了强大的超能力，能够用意念力制造建筑物，但是智商水平却没怎么增加。

现在 lqr 已经搞清楚黑暗城堡有 NN 个房间，MM 条可以制造的双向通道，以及每条通道的长度。

lqr 深知 Lord lsp 的想法，为了避免每次都要琢磨两个房间之间的最短路径，Lord lsp 一定会把城堡修建成树形的。

但是，为了尽量提高自己的移动效率，Lord lsp 一定会使得城堡满足下面的条件：

设 D\[i\]D\[i\] 为如果所有的通道都被修建，第 ii 号房间与第 11 号房间的最短路径长度；而 S\[i\]S\[i\] 为实际修建的树形城堡中第 ii 号房间与第 11 号房间的路径长度；要求对于所有整数 ii，有 S\[i\]\=D\[i\]S\[i\]\=D\[i\] 成立。

为了打败 Lord lsp，lqr 想知道有多少种不同的城堡修建方案。

保证至少存在一种可行的城堡修建方案。

你需要输出答案对 231–1231–1 取模之后的结果。

#### 输入格式

第一行有两个整数 NN 和 MM。

之后 MM 行，每行三个整数 X，YX，Y 和 LL，表示可以修建 XX 和 YY 之间的一条长度为 LL 的通道。

#### 输出格式

一个整数，表示答案对 231–1231–1 取模之后的结果。

#### 数据范围

2≤N≤10002≤N≤1000,  
N−1≤M≤N(N−1)/2N−1≤M≤N(N−1)/2,  
1≤L≤1001≤L≤100

#### 输入样例：

    3 3
    1 2 2
    1 3 1
    2 3 1
    

#### 输出样例：

    2
    

难度：困难

时/空限制：1s / 64MB

总通过数：1669

总尝试数：4512

来源：

[《算法竞赛进阶指南](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97&source_file_id=3900&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3900&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3900&show_algorithm_tags=1)[最小生成树](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E5%B0%8F%E7%94%9F%E6%88%90%E6%A0%91&source_file_id=3900&show_algorithm_tags=1)