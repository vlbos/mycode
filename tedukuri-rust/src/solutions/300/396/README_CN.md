396\. 矿场搭建

*    [题目](https://www.acwing.com/problem/content/description/398/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/398/1/)
*    [题解](https://www.acwing.com/problem/content/solution/398/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/398/)

  

煤矿工地可以看成是由隧道连接挖煤点组成的无向图。

为安全起见，希望在工地发生事故时所有挖煤点的工人都能有一条出路逃到救援出口处。

于是矿主决定在某些挖煤点设立救援出口，使得无论哪一个挖煤点坍塌之后，其他挖煤点的工人都有一条道路通向救援出口。

请写一个程序，用来计算至少需要设置几个救援出口，以及不同最少救援出口的设置方案总数。

#### 输入格式

输入文件有若干组数据，每组数据的第一行是一个正整数 NN，表示工地的隧道数。

接下来的 NN 行每行是用空格隔开的两个整数 SS 和 TT（S≠TS≠T），表示挖煤点 SS 与挖煤点 TT 由隧道直接连接。

注意，每组数据的挖煤点的编号为 1∼Max1∼Max，其中 MaxMax 表示由隧道连接的挖煤点中，编号最大的挖煤点的编号，可能存在没有被隧道连接的挖煤点。

输入数据以 00 结尾。

#### 输出格式

每组数据输出结果占一行。

其中第 ii 行以 `Case i:` 开始（注意大小写，`Case` 与 `i` 之间有空格，`i` 与 `:` 之间无空格，`:` 之后有空格）。

其后是用空格隔开的两个正整数，第一个正整数表示对于第 ii 组输入数据至少需要设置几个救援出口，第二个正整数表示对于第 ii 组输入数据不同最少救援出口的设置方案总数。

输入数据保证答案小于 264264，输出格式参照以下输入输出样例。

#### 数据范围

1≤N≤5001≤N≤500，  
1≤Max≤10001≤Max≤1000

#### 输入样例：

    9
    1  3
    4  1
    3  5
    1  2
    2  6
    1  5
    6  3
    1  6
    3  2
    6
    1  2
    1  3
    2  4
    2  5
    3  6
    3  7
    0
    

#### 输出样例：

    Case 1: 2 4
    Case 2: 4 1
    

难度：困难

时/空限制：1s / 64MB

总通过数：3696

总尝试数：8154

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3947&show_algorithm_tags=0)[HNOI2012](https://www.acwing.com/problem/search/1/?search_content=HNOI2012&source_file_id=3947&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3947&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3947&show_algorithm_tags=1)[Tarjan算法](https://www.acwing.com/problem/search/1/?search_content=Tarjan%E7%AE%97%E6%B3%95&source_file_id=3947&show_algorithm_tags=1)[割点](https://www.acwing.com/problem/search/1/?search_content=%E5%89%B2%E7%82%B9&source_file_id=3947&show_algorithm_tags=1)