144\. 最长异或值路径

*    [题目](https://www.acwing.com/problem/content/description/146/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/146/1/)
*    [题解](https://www.acwing.com/problem/content/solution/146/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/146/)

  

给定一个树，树上的边都具有权值。

树中一条路径的异或长度被定义为路径上所有边的权值的异或和：

![formula.png](https://cdn.acwing.com/media/article/image/2019/01/22/19_215873761d-formula.png)

⊕⊕ 为异或符号。

给定上述的具有 nn 个节点的树，你能找到异或长度最大的路径吗？

#### 输入格式

第一行包含整数 nn，表示树的节点数目。

接下来 n−1n−1 行，每行包括三个整数 u，v，wu，v，w，表示节点 uu 和节点 vv 之间有一条边权重为 ww。

#### 输出格式

输出一个整数，表示异或长度最大的路径的最大异或和。

#### 数据范围

1≤n≤1000001≤n≤100000,  
0≤u,v<n0≤u,v<n,  
0≤w<2310≤w<231

#### 输入样例：

    4
    0 1 3
    1 2 4
    1 3 6
    

#### 输出样例：

    7
    

#### 样例解释

样例中最长异或值路径应为 `0->1->2`，值为 7(\=3⊕4)7(\=3⊕4)

难度：中等

时/空限制：1s / 64MB

总通过数：4156

总尝试数：9934

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3695&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3695&show_algorithm_tags=0)

算法标签

[Trie](https://www.acwing.com/problem/search/1/?search_content=Trie&source_file_id=3695&show_algorithm_tags=1)[树上差分](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E4%B8%8A%E5%B7%AE%E5%88%86&source_file_id=3695&show_algorithm_tags=1)[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3695&show_algorithm_tags=1)