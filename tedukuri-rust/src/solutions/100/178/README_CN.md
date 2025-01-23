178\. 第K短路

*    [题目](https://www.acwing.com/problem/content/description/180/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/180/1/)
*    [题解](https://www.acwing.com/problem/content/solution/180/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/180/)

  

给定一张 NN 个点（编号 1,2…N1,2…N），MM 条边的有向图，求从起点 SS 到终点 TT 的第 KK 短路的长度，路径允许重复经过点或边。

**注意：** 每条最短路中至少要包含一条边。

#### 输入格式

第一行包含两个整数 NN 和 MM。

接下来 MM 行，每行包含三个整数 A,BA,B 和 LL，表示点 AA 与点 BB 之间存在有向边，且边长为 LL。

最后一行包含三个整数 S,TS,T 和 KK，分别表示起点 SS，终点 TT 和第 KK 短路。

#### 输出格式

输出占一行，包含一个整数，表示第 KK 短路的长度，如果第 KK 短路不存在，则输出 −1−1。

#### 数据范围

1≤S,T≤N≤10001≤S,T≤N≤1000,  
0≤M≤1040≤M≤104,  
1≤K≤10001≤K≤1000,  
1≤L≤1001≤L≤100

#### 输入样例：

    2 2
    1 2 5
    2 1 4
    1 2 2
    

#### 输出样例：

    14
    

难度：困难

时/空限制：1s / 64MB

总通过数：12088

总尝试数：33952

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3729&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3729&show_algorithm_tags=1)[A\*](https://www.acwing.com/problem/search/1/?search_content=A*&source_file_id=3729&show_algorithm_tags=1)[dijkstra](https://www.acwing.com/problem/search/1/?search_content=dijkstra&source_file_id=3729&show_algorithm_tags=1)[最短路](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E7%9F%AD%E8%B7%AF&source_file_id=3729&show_algorithm_tags=1)