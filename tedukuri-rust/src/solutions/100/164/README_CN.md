164\. 可达性统计

*    [题目](https://www.acwing.com/problem/content/description/166/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/166/1/)
*    [题解](https://www.acwing.com/problem/content/solution/166/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/166/)

  

给定一张 NN 个点 MM 条边的有向无环图，分别统计从每个点出发能够到达的点的数量。

#### 输入格式

第一行两个整数 N,MN,M，接下来 MM 行每行两个整数 x,yx,y，表示从 xx 到 yy 的一条有向边。

#### 输出格式

输出共 NN 行，表示每个点能够到达的点的数量。

#### 数据范围

1≤N,M≤300001≤N,M≤30000,  
1≤x,y≤N1≤x,y≤N

#### 输入样例：

    10 10
    3 8
    2 3
    2 5
    5 9
    5 9
    2 3
    3 9
    4 8
    2 10
    4 9
    

#### 输出样例：

    1
    6
    3
    3
    2
    1
    1
    1
    1
    1
    

难度：中等

时/空限制：1s / 256MB

总通过数：9764

总尝试数：16792

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3715&show_algorithm_tags=0)

算法标签

[拓扑排序](https://www.acwing.com/problem/search/1/?search_content=%E6%8B%93%E6%89%91%E6%8E%92%E5%BA%8F&source_file_id=3715&show_algorithm_tags=1)[位运算](https://www.acwing.com/problem/search/1/?search_content=%E4%BD%8D%E8%BF%90%E7%AE%97&source_file_id=3715&show_algorithm_tags=1)