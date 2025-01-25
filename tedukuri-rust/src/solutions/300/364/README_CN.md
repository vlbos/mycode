364\. 网络

*    [题目](https://www.acwing.com/problem/content/description/366/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/366/1/)
*    [题解](https://www.acwing.com/problem/content/solution/366/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/366/)

  

给定一张 NN 个点 MM 条边的无向连通图，然后执行 QQ 次操作，每次向图中添加一条边，并且询问当前无向图中“桥”的数量。

#### 输入格式

输入包含多组测试数据。

每组测试数据，第一行包含两个整数 NN 和 MM。

接下来 MM 行，每行包含两个整数 AA 和 BB，表示点 AA 和点 BB 之间有一条边，点的编号为 1∼N1∼N。

接下来一行，包含整数 QQ。

在接下来 QQ 行，每行包含两个整数 AA 和 BB，表示在 AA 和 BB 之间加一条边。

当输入 `0 0` 时表示输入终止。

#### 输出格式

每组数据第一行输出 `Case x:`，其中 xx 为组别编号，从 11 开始。

接下来 QQ 行，每行输出一个整数，表示一次询问的结果。

每组数据输出完毕后，输出一个空行。

#### 数据范围

1≤N≤1000001≤N≤100000  
N−1≤M≤200000N−1≤M≤200000,  
1≤A≠B≤N1≤A≠B≤N,  
1≤Q≤10001≤Q≤1000

#### 输入样例：

    3 2
    1 2
    2 3
    2
    1 2
    1 3
    4 4
    1 2
    2 1
    2 3
    1 4
    2
    1 2
    3 4
    0 0
    

#### 输出样例：

    Case 1:
    1
    0
    
    Case 2:
    2
    0
    
    

难度：中等

时/空限制：1s / 64MB

总通过数：1616

总尝试数：4344

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3915&show_algorithm_tags=0)[POJ3694](https://www.acwing.com/problem/search/1/?search_content=POJ3694&source_file_id=3915&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3915&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3915&show_algorithm_tags=1)[Tarjan算法](https://www.acwing.com/problem/search/1/?search_content=Tarjan%E7%AE%97%E6%B3%95&source_file_id=3915&show_algorithm_tags=1)[无向图的双连通分量](https://www.acwing.com/problem/search/1/?search_content=%E6%97%A0%E5%90%91%E5%9B%BE%E7%9A%84%E5%8F%8C%E8%BF%9E%E9%80%9A%E5%88%86%E9%87%8F&source_file_id=3915&show_algorithm_tags=1)