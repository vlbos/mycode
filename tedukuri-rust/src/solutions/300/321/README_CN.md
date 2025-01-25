321\. 棋盘分割

*    [题目](https://www.acwing.com/problem/content/description/323/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/323/1/)
*    [题解](https://www.acwing.com/problem/content/solution/323/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/323/)

  

将一个 8×88×8 的棋盘进行如下分割：将原棋盘割下一块矩形棋盘并使剩下部分也是矩形，再将剩下的部分继续如此分割，这样割了 (n−1)(n−1) 次后，连同最后剩下的矩形棋盘共有 nn 块矩形棋盘。(每次切割都只能沿着棋盘格子的边进行)

![1191_1.jpg](https://cdn.acwing.com/media/article/image/2019/02/05/19_32dad08629-1191_1.jpg)

原棋盘上每一格有一个分值，一块矩形棋盘的总分为其所含各格分值之和。

现在需要把棋盘按上述规则分割成 nn 块矩形棋盘，并使各矩形棋盘总分的均方差最小。

均方差![formula.png](https://cdn.acwing.com/media/article/image/2019/02/05/19_566d096029-formula.png) ，其中平均值![lala.png](https://cdn.acwing.com/media/article/image/2019/02/05/19_047fe57229-lala.png) ，xixi 为第 ii 块矩形棋盘的总分。

请编程对给出的棋盘及 nn，求出均方差的最小值。

#### 输入格式

第 11 行为一个整数 nn。

第 22 行至第 99 行每行为 88 个小于 100100 的非负整数，表示棋盘上相应格子的分值。每行相邻两数之间用一个空格分隔。

#### 输出格式

输出最小均方差值（四舍五入精确到小数点后三位）。

#### 数据范围

1<n<151<n<15

#### 输入样例：

    3
    1 1 1 1 1 1 1 3
    1 1 1 1 1 1 1 1
    1 1 1 1 1 1 1 1
    1 1 1 1 1 1 1 1
    1 1 1 1 1 1 1 1
    1 1 1 1 1 1 1 1
    1 1 1 1 1 1 1 0
    1 1 1 1 1 1 0 3
    

#### 输出样例：

    1.633
    

难度：中等

时/空限制：1s / 10MB

总通过数：7421

总尝试数：11934

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3872&show_algorithm_tags=0)[NOI1999](https://www.acwing.com/problem/search/1/?search_content=NOI1999&source_file_id=3872&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3872&show_algorithm_tags=1)[区间DP](https://www.acwing.com/problem/search/1/?search_content=%E5%8C%BA%E9%97%B4DP&source_file_id=3872&show_algorithm_tags=1)