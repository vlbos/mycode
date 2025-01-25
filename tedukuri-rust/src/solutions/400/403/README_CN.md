403\. 平面

*    [题目](https://www.acwing.com/problem/content/description/405/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/405/1/)
*    [题解](https://www.acwing.com/problem/content/solution/405/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/405/)

  

若能将无向图 G\=(V,E)G\=(V,E) 画在平面上使得任意两条无重合顶点的边不相交，则称 GG 是平面图。

判定一个图是否为平面图的问题是图论中的一个重要问题。

现在假设你要判定的是一类特殊的图，图中存在一个包含所有顶点的环，即存在哈密顿回路。

请你判定它们是否是平面图。

#### 输入格式

第一行包含正整数 TT，表示共有 TT 组测试数据。

每组测试数据第一行包含两个整数 NN 和 MM，分别表示对应图的顶点数和边数。

之后 MM 行，每行包含两个整数 uu 和 vv，表示对应图的一条边 (u,v)(u,v)，输入数据保证所有边仅出现一次。

最后一行，包含 NN 个整数，从左到右表示对应图中的一个哈密顿回路。

#### 输出格式

输出共 TT 行。

如果第 ii 组数据对应的图是平面图，则第 ii 行输出 `YES`，否则输出 `NO`。

#### 数据范围

T≤100,3≤N≤200,M≤10000T≤100,3≤N≤200,M≤10000

#### 输入样例：

    2 
    6 9 
    1 4 
    1 5 
    1 6 
    2 4 
    2 5 
    2 6 
    3 4 
    3 5 
    3 6 
    1 4 2 5 3 6 
    5 5 
    1 2 
    2 3 
    3 4 
    4 5 
    5 1 
    1 2 3 4 5
    

#### 输出样例：

    NO
    YES
    

难度：困难

时/空限制：1s / 64MB

总通过数：313

总尝试数：968

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3954&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3954&show_algorithm_tags=1)[2-SAT](https://www.acwing.com/problem/search/1/?search_content=2-SAT&source_file_id=3954&show_algorithm_tags=1)