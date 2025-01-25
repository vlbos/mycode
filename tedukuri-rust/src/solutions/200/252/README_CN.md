252\. 树

*    [题目](https://www.acwing.com/problem/content/description/254/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/254/1/)
*    [题解](https://www.acwing.com/problem/content/solution/254/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/254/)

  

给定一个有 NN 个点（编号 0,1,…,N−10,1,…,N−1）的树，每条边都有一个权值（不超过 10001000）。

树上两个节点 xx 与 yy 之间的路径长度就是路径上各条边的权值之和。

求长度不超过 KK 的路径有多少条。

#### 输入格式

输入包含多组测试用例。

每组测试用例的第一行包含两个整数 NN 和 KK。

接下来 N−1N−1 行，每行包含三个整数 u,v,lu,v,l，表示节点 uu 与 vv 之间存在一条边，且边的权值为 ll。

当输入用例 N\=0，K\=0N\=0，K\=0 时，表示输入终止，且该用例无需处理。

#### 输出格式

每个测试用例输出一个结果。

每个结果占一行。

#### 数据范围

1≤N≤1041≤N≤104,  
1≤K≤5×1061≤K≤5×106,  
0≤l≤1030≤l≤103

#### 输入样例：

    5 4
    0 1 3
    0 2 1
    0 3 2
    2 4 1
    0 0
    

#### 输出样例：

    8
    

难度：困难

时/空限制：0.3s / 64MB

总通过数：3433

总尝试数：8782

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3803&show_algorithm_tags=0)[男人八题](https://www.acwing.com/problem/search/1/?search_content=%E7%94%B7%E4%BA%BA%E5%85%AB%E9%A2%98&source_file_id=3803&show_algorithm_tags=0)

算法标签

[点分治](https://www.acwing.com/problem/search/1/?search_content=%E7%82%B9%E5%88%86%E6%B2%BB&source_file_id=3803&show_algorithm_tags=1)