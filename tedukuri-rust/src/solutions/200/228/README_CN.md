228\. 异或

*    [题目](https://www.acwing.com/problem/content/description/230/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/230/1/)
*    [题解](https://www.acwing.com/problem/content/solution/230/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/230/)

  

两个非负整数的 XORXOR 是指将它们表示成二进制数，再在对应的二进制位进行 XORXOR 运算。

现在，定义 KK 个非负整数 A1,A2,…,AKA1,A2,…,AK 的 XORXOR 和为：

A1 XOR A2 XOR … XOR AKA1 XOR A2 XOR … XOR AK

考虑一个边权为非负整数的无向连通图，节点编号 11 到 NN，试求出一条从 11 号节点到 NN 号节点的路径，使路径上经过的边的权值的 XORXOR 和最大。

路径可以重复经过某些点或边，当一条边在路径中出现了多次时，其权值在计算 XORXOR 和时也要被计算相应多的次数。

#### 输入格式

第一行包含两个整数 NN 和 MM，表示该无向图中点的数目与边的数目。

接下来 MM 行描述 MM 条边，每行三个整数 Si，Ti，DiSi，Ti，Di，表示 SiSi 与 TiTi 之间存在一条权值为 DiDi 的无向边。

图中可能有重边或自环。

#### 输出格式

仅包含一个整数，表示最大的 XORXOR 和（十进制结果），注意输出后加换行回车。

#### 数据范围

N≤50000,M≤100000,Di≤1018N≤50000,M≤100000,Di≤1018

#### 输入样例：

    5 7 
    1 2 2 
    1 3 2 
    2 4 1 
    2 5 1 
    4 5 3 
    5 3 4 
    4 3 2 
    

#### 输出样例：

    6
    

难度：中等

时/空限制：1s / 64MB

总通过数：529

总尝试数：1307

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3779&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3779&show_algorithm_tags=1)[线性空间](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7%E7%A9%BA%E9%97%B4&source_file_id=3779&show_algorithm_tags=1)[高斯消元](https://www.acwing.com/problem/search/1/?search_content=%E9%AB%98%E6%96%AF%E6%B6%88%E5%85%83&source_file_id=3779&show_algorithm_tags=1)[线性基](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7%E5%9F%BA&source_file_id=3779&show_algorithm_tags=1)