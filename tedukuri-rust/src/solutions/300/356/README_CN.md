356\. 次小生成树

*    [题目](https://www.acwing.com/problem/content/description/358/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/358/1/)
*    [题解](https://www.acwing.com/problem/content/solution/358/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/358/)

  

给定一张 NN 个点 MM 条边的无向图，求无向图的严格次小生成树。

设最小生成树的边权之和为 sumsum，严格次小生成树就是指边权之和大于 sumsum 的生成树中最小的一个。

#### 输入格式

第一行包含两个整数 NN 和 MM。

接下来 MM 行，每行包含三个整数 x，y，zx，y，z，表示点 xx 和点 yy 之前存在一条边，边的权值为 zz。

#### 输出格式

包含一行，仅一个数，表示严格次小生成树的边权和。(数据保证必定存在严格次小生成树)

#### 数据范围

N≤105,M≤3×105N≤105,M≤3×105,  
1≤x,y≤N1≤x,y≤N,  
0≤z≤1060≤z≤106

#### 输入样例：

    5 6
    1 2 1
    1 3 2
    2 4 3
    3 5 4
    3 4 3
    4 5 6
    

#### 输出样例：

    11
    

难度：困难

时/空限制：2s / 512MB

总通过数：8540

总尝试数：28555

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3907&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3907&show_algorithm_tags=1)[LCA的综合应用](https://www.acwing.com/problem/search/1/?search_content=LCA%E7%9A%84%E7%BB%BC%E5%90%88%E5%BA%94%E7%94%A8&source_file_id=3907&show_algorithm_tags=1)