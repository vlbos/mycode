351\. 树网的核

*    [题目](https://www.acwing.com/problem/content/description/353/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/353/1/)
*    [题解](https://www.acwing.com/problem/content/solution/353/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/353/)

  

设 T\=(V,E,W)T\=(V,E,W) 是一个无圈且连通的无向图（也称为无根树），每条边带有正整数的权，我们称 TT 为树网（treenetwork），其中 V,EV,E 分别表示结点与边的集合，WW 表示各边长度的集合，并设 TT 有 nn 个结点。

路径：树网中任何两结点 a,ba,b 都存在唯一的一条简单路径，用 d(a,b)d(a,b) 表示以 a,ba,b 为端点的路径的长度，它是该路径上各边长度之和。

我们称 d(a,b)d(a,b) 为 a,ba,b 两结点间的距离。

一点 vv 到一条路径 PP 的距离为该点与 PP 上的最近的结点的距离：

d(v，P)\=min{d(v，u)}d(v，P)\=min{d(v，u)}，uu 为路径 PP 上的结点。

树网的直径：树网中最长的路径称为树网的直径。

对于给定的树网 TT，直径不一定是唯一的，但可以证明：各直径的中点（不一定恰好是某个结点，可能在某条边的内部）是唯一的，我们称该点为树网的中心。

偏心距 ECC(F)ECC(F)：树网 TT 中距路径 FF 最远的结点到路径 FF 的距离，即：

ECC(F)\=max{d(v,F),v∈V}ECC(F)\=max{d(v,F),v∈V}

任务：对于给定的树网 T\=(V,E,W)T\=(V,E,W) 和非负整数 ss，求一个路径 FF，它是某直径上的一段路径（该路径两端均为树网中的结点），其长度不超过 ss（可以等于 ss），使偏心距 ECC(F)ECC(F) 最小。

我们称这个路径为树网 T\=(V,E,W)T\=(V,E,W) 的核（Core）。

必要时，FF 可以退化为某个结点。

一般来说，在上述定义下，核不一定只有一个，但最小偏心距是唯一的。

#### 输入格式

包含 nn 行： 第 11 行，两个正整数 nn 和 ss，中间用一个空格隔开，其中 nn 为树网结点的个数，ss 为树网的核的长度的上界，设结点编号依次为 1,2,…,n1,2,…,n。

从第 22 行到第 nn 行，每行给出 33 个用空格隔开的正整数，依次表示每一条边的两个端点编号和长度。

例如，`2 4 7` 表示连接结点 22 与 44 的边的长度为 77。

所给的数据都是正确的，不必检验。

#### 输出格式

只有一个非负整数，为指定意义下的最小偏心距。

#### 数据范围

1≤n≤5000001≤n≤500000  
0≤s<2310≤s<231  
0≤树的直径长度<2310≤树的直径长度<231

#### 输入样例：

    5 2
    1 2 5
    2 3 2
    2 4 4
    2 5 3
    

#### 输出样例：

    5
    

难度：困难

时/空限制：1s / 128MB

总通过数：1299

总尝试数：4673

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3902&show_algorithm_tags=0)[NOIP2007提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2007%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3902&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3902&show_algorithm_tags=1)[树的直径](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E7%9A%84%E7%9B%B4%E5%BE%84&source_file_id=3902&show_algorithm_tags=1)[二分](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86&source_file_id=3902&show_algorithm_tags=1)[单调队列](https://www.acwing.com/problem/search/1/?search_content=%E5%8D%95%E8%B0%83%E9%98%9F%E5%88%97&source_file_id=3902&show_algorithm_tags=1)[双指针](https://www.acwing.com/problem/search/1/?search_content=%E5%8F%8C%E6%8C%87%E9%92%88&source_file_id=3902&show_algorithm_tags=1)