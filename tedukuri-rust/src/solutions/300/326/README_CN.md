326\. XOR和路径

*    [题目](https://www.acwing.com/problem/content/description/328/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/328/1/)
*    [题解](https://www.acwing.com/problem/content/solution/328/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/328/)

  

给定一个无向连通图，其节点编号为 11 到 NN，其边的权值为非负整数。

试求出一条从 11 号节点都 NN 号节点的路径，使得该路径上经过的边的权值的 XORXOR 和最大。

该路径可以重复经过某些节点或边，当一条边在路径中出现多次时，其权值在计算 XORXOR 和时也应被重复计算相应多的次数。

直接求解上述问题比较困难，于是你决定使用非完美算法。

具体来说，从 11 号节点开始，以相等的概率，随机选择与当前节点相关联的某条边，并沿着这条边走到下一个节点，重复这个过程直到走到 NN 号节点为止，便得到一条从 11 号节点到 NN 号节点的路径。

显然得到每条这样的路径的概率是不同的，并且每条这样的路径的 XORXOR 和也不一样。

现在请你求出该算法得到的路径的 XORXOR 和的期望值。

#### 输入格式

第一行包含两个整数 NN 和 MM，表示节点数和边数。

接下来 MM 行，每行包含三个整数 u,v,wu,v,w，表示存在一条边 (u,v)(u,v)，权值为 ww。

图中可能存在重边或自环。

#### 输出格式

输出包含一个实数，表示 XORXOR 和的期望值，结果保留三位小数。

#### 数据范围

2≤N≤1002≤N≤100,  
M≤10000M≤10000,  
1≤u,v≤N1≤u,v≤N,  
0≤w≤1090≤w≤109

#### 输入样例：

    2 2
    1 1 2
    1 2 3
    

#### 输出样例：

    2.333
    

难度：中等

时/空限制：1s / 64MB

总通过数：385

总尝试数：843

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3877&show_algorithm_tags=0)[HNOI2011](https://www.acwing.com/problem/search/1/?search_content=HNOI2011&source_file_id=3877&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3877&show_algorithm_tags=1)[有后效性DP](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%89%E5%90%8E%E6%95%88%E6%80%A7DP&source_file_id=3877&show_algorithm_tags=1)[高斯消元](https://www.acwing.com/problem/search/1/?search_content=%E9%AB%98%E6%96%AF%E6%B6%88%E5%85%83&source_file_id=3877&show_algorithm_tags=1)[数学期望](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E6%9C%9F%E6%9C%9B&source_file_id=3877&show_algorithm_tags=1)