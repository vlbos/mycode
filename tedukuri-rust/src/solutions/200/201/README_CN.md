201\. 可见的点

*    [题目](https://www.acwing.com/problem/content/description/203/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/203/1/)
*    [题解](https://www.acwing.com/problem/content/solution/203/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/203/)

  

在一个平面直角坐标系的第一象限内，如果一个点 (x,y)(x,y) 与原点 (0,0)(0,0) 的连线中没有通过其他任何点，则称该点在原点处是可见的。

例如，点 (4,2)(4,2) 就是不可见的，因为它与原点的连线会通过点 (2,1)(2,1)。

部分可见点与原点的连线如下图所示：

![3090_1.png](https://cdn.acwing.com/media/article/image/2019/01/18/19_a68c1a281a-3090_1.png)

编写一个程序，计算给定整数 NN 的情况下，满足 0≤x，y≤N0≤x，y≤N 的可见点 (x，y)(x，y) 的数量（可见点不包括原点）。

#### 输入格式

第一行包含整数 CC，表示共有 CC 组测试数据。

每组测试数据占一行，包含一个整数 NN。

#### 输出格式

每组测试数据的输出占据一行。

应包括：测试数据的编号（从 11 开始），该组测试数据对应的 NN 以及可见点的数量。

同行数据之间用空格隔开。

#### 数据范围

1≤N,C≤10001≤N,C≤1000

#### 输入样例：

    4
    2
    4
    5
    231
    

#### 输出样例：

    1 2 5
    2 4 13
    3 5 21
    4 231 32549
    

难度：简单

时/空限制：1s / 64MB

总通过数：7034

总尝试数：10679

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3752&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3752&show_algorithm_tags=1)[欧拉函数](https://www.acwing.com/problem/search/1/?search_content=%E6%AC%A7%E6%8B%89%E5%87%BD%E6%95%B0&source_file_id=3752&show_algorithm_tags=1)