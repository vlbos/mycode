367\. 学校网络

*    [题目](https://www.acwing.com/problem/content/description/369/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/369/1/)
*    [题解](https://www.acwing.com/problem/content/solution/369/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/369/)

  

一些学校连接在一个计算机网络上，学校之间存在软件支援协议，每个学校都有它应支援的学校名单（学校 AA 支援学校 BB，并不表示学校 BB 一定要支援学校 AA）。

当某校获得一个新软件时，无论是直接获得还是通过网络获得，该校都应立即将这个软件通过网络传送给它应支援的学校。

因此，一个新软件若想让所有学校都能使用，只需将其提供给一些学校即可。

现在请问最少需要将一个新软件直接提供给多少个学校，才能使软件能够通过网络被传送到所有学校？

最少需要添加几条新的支援关系，使得将一个新软件提供给任何一个学校，其他所有学校就都可以通过网络获得该软件？

#### 输入格式

第 11 行包含整数 NN，表示学校数量。

第 2..N+12..N+1 行，每行包含一个或多个整数，第 i+1i+1 行表示学校 ii 应该支援的学校名单，每行最后都有一个 00 表示名单结束（只有一个 00 即表示该学校没有需要支援的学校）。

#### 输出格式

输出两个问题的结果，每个结果占一行。

#### 数据范围

2≤N≤1002≤N≤100

#### 输入样例：

    5
    2 4 3 0
    4 5 0
    0
    0
    1 0
    

#### 输出样例：

    1
    2
    

难度：中等

时/空限制：1s / 64MB

总通过数：7275

总尝试数：16491

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3918&show_algorithm_tags=0)[usaco training 5.3](https://www.acwing.com/problem/search/1/?search_content=usaco%20training%205.3&source_file_id=3918&show_algorithm_tags=0)[POJ1236](https://www.acwing.com/problem/search/1/?search_content=POJ1236&source_file_id=3918&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3918&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3918&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3918&show_algorithm_tags=1)[Tarjan算法](https://www.acwing.com/problem/search/1/?search_content=Tarjan%E7%AE%97%E6%B3%95&source_file_id=3918&show_algorithm_tags=1)[有向图的强连通分量](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%89%E5%90%91%E5%9B%BE%E7%9A%84%E5%BC%BA%E8%BF%9E%E9%80%9A%E5%88%86%E9%87%8F&source_file_id=3918&show_algorithm_tags=1)