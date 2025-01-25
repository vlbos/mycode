217\. 绿豆蛙的归宿

*    [题目](https://www.acwing.com/problem/content/description/219/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/219/1/)
*    [题解](https://www.acwing.com/problem/content/solution/219/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/219/)

  

给出一个有向无环的连通图，起点为 11，终点为 NN，每条边都有一个长度。

数据保证从起点出发能够到达图中所有的点，图中所有的点也都能够到达终点。

绿豆蛙从起点出发，走向终点。

到达每一个顶点时，如果有 KK 条离开该点的道路，绿豆蛙可以选择任意一条道路离开该点，并且走向每条路的概率为 1/K1/K。

现在绿豆蛙想知道，从起点走到终点所经过的路径总长度的期望是多少？

#### 输入格式

第一行: 两个整数 N，MN，M，代表图中有 NN 个点、MM 条边。

第二行到第 1+M1+M 行: 每行 33 个整数 a,b,ca,b,c，代表从 aa 到 bb 有一条长度为 cc 的有向边。

#### 输出格式

输出从起点到终点路径总长度的期望值，结果四舍五入保留两位小数。

#### 数据范围

1≤N≤1051≤N≤105,  
1≤M≤2N1≤M≤2N

#### 输入样例：

    4 4
    1 2 1
    1 3 2
    2 3 3
    3 4 4
    

#### 输出样例：

    7.00
    

难度：简单

时/空限制：1s / 64MB

总通过数：4975

总尝试数：7541

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3768&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3768&show_algorithm_tags=1)[概率与数学期望](https://www.acwing.com/problem/search/1/?search_content=%E6%A6%82%E7%8E%87%E4%B8%8E%E6%95%B0%E5%AD%A6%E6%9C%9F%E6%9C%9B&source_file_id=3768&show_algorithm_tags=1)