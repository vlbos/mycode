322\. 消木块

*    [题目](https://www.acwing.com/problem/content/description/324/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/324/1/)
*    [题解](https://www.acwing.com/problem/content/solution/324/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/324/)

  

你们中的一些人可能玩过一个叫做消木块的游戏。

nn 个木块排成一列，每个木块都有一个颜色。

例如下图中木块的颜色分别为：金，银，银，银，银，铜，铜，铜，金。

![1390_1.jpg](https://cdn.acwing.com/media/article/image/2019/02/17/19_e646835a32-1390_1.jpg)

每次，你都可以点击一个木块，这样被点击的木块以及和它相邻并且同色的木块就会消除。

如果一次性消除了 kk 个木块，那么就会得到 k×kk×k 分。

例如下图所示，点击银色木块，四个木块被消去，得到 1616 分。

![1390_2.jpg](https://cdn.acwing.com/media/article/image/2019/02/17/19_7d967bd432-1390_2.jpg)

给定你一个游戏初始状态，请你求出最高得分是多少。

#### 输入格式

第一行包含整数 tt，表示共有 tt 组测试数据。

每组数据第一行包含整数 nn，表示共有 nn 个木块。

第二行包含 nn 个整数，表示 nn 个木块的颜色。

代表木块颜色的整数范围是 1∼n1∼n。

#### 输出格式

每组数据输出一个结果，每个结果占一行。

输出格式为 `Case x: y`，其中 xx 为数据组别编号，从 11 开始，yy 为结果。

#### 数据范围

1≤n≤2001≤n≤200

#### 输入样例：

    2
    9
    1 2 2 2 2 3 3 3 1
    1
    1
    

#### 输出样例：

    Case 1: 29
    Case 2: 1
    

难度：困难

时/空限制：1s / 64MB

总通过数：671

总尝试数：1615

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3873&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3873&show_algorithm_tags=1)[区间DP](https://www.acwing.com/problem/search/1/?search_content=%E5%8C%BA%E9%97%B4DP&source_file_id=3873&show_algorithm_tags=1)