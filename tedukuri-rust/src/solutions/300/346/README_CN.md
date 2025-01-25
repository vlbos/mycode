346\. 走廊泼水节

*    [题目](https://www.acwing.com/problem/content/description/348/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/348/1/)
*    [题解](https://www.acwing.com/problem/content/solution/348/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/348/)

  

给定一棵 NN 个节点的树，要求增加若干条边，把这棵树扩充为完全图，并满足图的唯一最小生成树仍然是这棵树。

求增加的边的权值总和最小是多少。

**注意：** 树中的所有边权均为整数，且新加的所有边权也必须为整数。

#### 输入格式

第一行包含整数 tt，表示共有 tt 组测试数据。

对于每组测试数据，第一行包含整数 NN。

接下来 N−1N−1 行，每行三个整数 X,Y,ZX,Y,Z，表示 XX 节点与 YY 节点之间存在一条边，长度为 ZZ。

#### 输出格式

每组数据输出一个整数，表示权值总和最小值。

每个结果占一行。

#### 数据范围

1≤N≤60001≤N≤6000  
1≤Z≤1001≤Z≤100

#### 输入样例：

    2
    3
    1 2 2
    1 3 3
    4
    1 2 3
    2 3 4
    3 4 5 
    

#### 输出样例：

    4
    17 
    

难度：中等

时/空限制：1s / 64MB

总通过数：8025

总尝试数：13156

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3897&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3897&show_algorithm_tags=1)[最小生成树](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E5%B0%8F%E7%94%9F%E6%88%90%E6%A0%91&source_file_id=3897&show_algorithm_tags=1)[Kruskal](https://www.acwing.com/problem/search/1/?search_content=Kruskal&source_file_id=3897&show_algorithm_tags=1)