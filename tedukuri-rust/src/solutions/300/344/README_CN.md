344\. 观光之旅

*    [题目](https://www.acwing.com/problem/content/description/346/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/346/1/)
*    [题解](https://www.acwing.com/problem/content/solution/346/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/346/)

  

给定一张无向图，求图中一个至少包含 33 个点的环，环上的节点不重复，并且环上的边的长度之和最小。

该问题称为无向图的最小环问题。

你需要输出最小环的方案，若最小环不唯一，输出任意一个均可。

#### 输入格式

第一行包含两个整数 NN 和 MM，表示无向图有 NN 个点，MM 条边。

接下来 MM 行，每行包含三个整数 u，v，lu，v，l，表示点 uu 和点 vv 之间有一条边，边长为 ll。

#### 输出格式

输出占一行，包含最小环的所有节点（按顺序输出），如果不存在则输出 `No solution.`。

#### 数据范围

1≤N≤1001≤N≤100,  
1≤M≤100001≤M≤10000,  
1≤l<5001≤l<500

#### 输入样例：

    5 7
    1 4 1
    1 3 300
    3 1 10
    1 2 16
    2 3 100
    2 5 15
    5 3 20
    

#### 输出样例：

    1 3 5 2
    

难度：中等

时/空限制：1s / 64MB

总通过数：8437

总尝试数：18806

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3895&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3895&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3895&show_algorithm_tags=1)[Floyd](https://www.acwing.com/problem/search/1/?search_content=Floyd&source_file_id=3895&show_algorithm_tags=1)