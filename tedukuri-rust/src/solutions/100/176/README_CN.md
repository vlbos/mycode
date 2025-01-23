176\. 装满的油箱

*    [题目](https://www.acwing.com/problem/content/description/178/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/178/1/)
*    [题解](https://www.acwing.com/problem/content/solution/178/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/178/)

  

有 NN 个城市（编号 0、1…N−10、1…N−1）和 MM 条道路，构成一张无向图。

在每个城市里边都有一个加油站，不同的加油站的单位油价不一样。

现在你需要回答不超过 100100 个问题，在每个问题中，请计算出一架油箱容量为 CC 的车子，从起点城市 SS 开到终点城市 EE 至少要花多少油钱？

**注意：** 假定车子初始时油箱是空的。

#### 输入格式

第一行包含两个整数 NN 和 MM。

第二行包含 NN 个整数，代表 NN 个城市的单位油价，第 ii 个数即为第 ii 个城市的油价 pipi。

接下来 MM 行，每行包括三个整数 u,v,du,v,d，表示城市 uu 与城市 vv 之间存在道路，且车子从 uu 到 vv 需要消耗的油量为 dd。

接下来一行包含一个整数 qq，代表问题数量。

接下来 qq 行，每行包含三个整数 C、S、EC、S、E，分别表示车子油箱容量 CC、起点城市 SS、终点城市 EE。

#### 输出格式

对于每个问题，输出一个整数，表示所需的最少油钱。

如果无法从起点城市开到终点城市，则输出 `impossible`。

每个结果占一行。

#### 数据范围

1≤N≤10001≤N≤1000,  
1≤M≤100001≤M≤10000,  
1≤pi≤1001≤pi≤100,  
1≤d≤1001≤d≤100,  
1≤C≤1001≤C≤100,  
1≤q≤1001≤q≤100。

#### 输入样例：

    5 5
    10 10 20 12 13
    0 1 9
    0 2 8
    1 2 1
    1 3 11
    2 3 7
    2
    10 0 3
    20 1 4
    

#### 输出样例：

    170
    impossible
    

难度：中等

时/空限制：2s / 64MB

总通过数：2699

总尝试数：7336

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3727&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3727&show_algorithm_tags=1)[优先队列BFS](https://www.acwing.com/problem/search/1/?search_content=%E4%BC%98%E5%85%88%E9%98%9F%E5%88%97BFS&source_file_id=3727&show_algorithm_tags=1)