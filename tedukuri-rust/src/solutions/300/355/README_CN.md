355\. 异象石

*    [题目](https://www.acwing.com/problem/content/description/357/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/357/1/)
*    [题解](https://www.acwing.com/problem/content/solution/357/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/357/)

  

Adera 是 Microsoft 应用商店中的一款解谜游戏。

异象石是进入 Adera 中异时空的引导物，在 Adera 的异时空中有一张地图。

这张地图上有 NN 个点，有 N−1N−1 条双向边把它们连通起来。

起初地图上没有任何异象石，在接下来的 MM 个时刻中，每个时刻会发生以下三种类型的事件之一：

1.  地图的某个点上出现了异象石（已经出现的不会再次出现）；
2.  地图某个点上的异象石被摧毁（不会摧毁没有异象石的点）；
3.  向玩家询问使所有异象石所在的点连通的边集的总长度最小是多少。

请你作为玩家回答这些问题。

#### 输入格式

第一行有一个整数 NN，表示点的个数。

接下来 N−1N−1 行每行三个整数 x,y,zx,y,z，表示点 xx 和 yy 之间有一条长度为 zz 的双向边。

第 N+1N+1 行有一个正整数 MM。

接下来 MM 行每行是一个事件，事件是以下三种格式之一：

*   `+ x` 表示点 xx 上出现了异象石
*   `- x` 表示点 xx 上的异象石被摧毁
*   `?` 表示询问使当前所有异象石所在的点连通所需的边集的总长度最小是多少。

#### 输出格式

对于每个 `?` 事件，输出一个整数表示答案。

#### 数据范围

1≤N,M≤1051≤N,M≤105,  
1≤x,y≤N1≤x,y≤N,  
x≠yx≠y,  
1≤z≤1091≤z≤109

#### 输入样例：

    6
    1 2 1
    1 3 5
    4 1 7
    4 5 3
    6 4 2
    10
    + 3
    + 1
    ?
    + 6
    ?
    + 5
    ?
    - 6
    - 3
    ?
    

#### 输出样例：

    5
    14
    17
    10
    

难度：困难

时/空限制：1s / 128MB

总通过数：1143

总尝试数：3102

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3906&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3906&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3906&show_algorithm_tags=1)[LCA的综合应用](https://www.acwing.com/problem/search/1/?search_content=LCA%E7%9A%84%E7%BB%BC%E5%90%88%E5%BA%94%E7%94%A8&source_file_id=3906&show_algorithm_tags=1)