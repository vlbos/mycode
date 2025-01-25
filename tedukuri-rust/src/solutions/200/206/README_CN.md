206\. 石头游戏

*    [题目](https://www.acwing.com/problem/content/description/208/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/208/1/)
*    [题解](https://www.acwing.com/problem/content/solution/208/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/208/)

  

石头游戏在一个 nn 行 mm 列的网格上进行，每个格子对应一种操作序列，操作序列至多有 1010 种，分别用 0∼90∼9 这 1010 个数字指明。

操作序列是一个长度不超过 66 且循环执行、每秒执行一个字符的字符串。

每秒钟，所有格子同时执行各自操作序列里的下一个字符。

序列中的每个字符是以下格式之一：

1.  数字 0∼90∼9：表示拿 0∼90∼9 个石头到该格子。
2.  `NWSE`：表示把这个格子内所有的石头推到相邻的格子，`N` 表示上方，`W` 表示左方，`S` 表示下方，`E` 表示右方。
3.  `D`：表示拿走这个格子的所有石头。

给定每种操作序列对应的字符串，以及网格中每个格子对应的操作序列，求石头游戏进行了 tt 秒之后，石头最多的格子里有多少个石头。

在游戏开始时，网格是空的。

#### 输入格式

第一行 44 个整数 n,m,t,actn,m,t,act。

接下来 nn 行，每行 mm 个字符，表示每个格子对应的操作序列。

最后 actact 行，每行一个字符串，表示从 00 开始的每个操作序列。

#### 输出格式

一个整数：游戏进行了 tt 秒之后，所有方格中石头最多的格子有多少个石头。

#### 数据范围

1≤m,n≤81≤m,n≤8,  
1≤t≤1081≤t≤108,  
1≤act≤101≤act≤10

#### 输入样例：

    1 6 10 3
    011112
    1E
    E
    0
    

#### 输出样例：

    3
    

#### 样例解释

样例中给出了三组操作序列，第一个格子执行编号为 00 的操作序列 `1E`，第二至五个格子执行编号为 11 的操作序列 `E`，第六个格子执行编号为 22 的操作序列 `0`。

这是另一个类似于传送带的结构，左边的设备 00 间隔地产生石头并向东传送。

设备 11 向右传送，直到设备 22。

1010 秒后，总共产生了 55 个石头，22 个在传送带上，33 个在最右边。

难度：简单

时/空限制：1s / 64MB

总通过数：1129

总尝试数：2832

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3757&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3757&show_algorithm_tags=1)[矩阵乘法](https://www.acwing.com/problem/search/1/?search_content=%E7%9F%A9%E9%98%B5%E4%B9%98%E6%B3%95&source_file_id=3757&show_algorithm_tags=1)