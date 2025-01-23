115\. 给树染色

*    [题目](https://www.acwing.com/problem/content/description/117/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/117/1/)
*    [题解](https://www.acwing.com/problem/content/solution/117/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/117/)

  

一颗树有 nn 个节点，这些节点被标号为：1,2,3…n1,2,3…n，每个节点 ii 都有一个权值 A\[i\]A\[i\]。

现在要把这棵树的节点全部染色，染色的规则是：

根节点 RR 可以随时被染色；对于其他节点，在被染色之前它的父亲节点必须已经染上了色。

每次染色的代价为 T×A\[i\]T×A\[i\]，其中 TT 代表当前是第几次染色。

求把这棵树染色的最小总代价。

#### 输入格式

第一行包含两个整数 nn 和 RR，分别代表树的节点数以及根节点的序号。

第二行包含 nn 个整数，代表所有节点的权值，第 ii 个数即为第 ii 个节点的权值 A\[i\]A\[i\]。

接下来 n−1n−1 行，每行包含两个整数 aa 和 bb，代表两个节点的序号，两节点满足关系： aa 节点是 bb 节点的父节点。

除根节点外的其他 n−1n−1 个节点的父节点和它们本身会在这 n−1n−1 行中表示出来。

同一行内的数用空格隔开。

#### 输出格式

输出一个整数，代表把这棵树染色的最小总代价。

#### 数据范围

1≤n≤10001≤n≤1000,  
1≤A\[i\]≤10001≤A\[i\]≤1000

#### 输入样例：

    5 1
    1 2 1 2 4
    1 2
    1 3
    2 4
    3 5
    

#### 输出样例：

    33
    

难度：困难

时/空限制：1s / 64MB

总通过数：2948

总尝试数：6054

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3666&show_algorithm_tags=0)

算法标签

[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3666&show_algorithm_tags=1)