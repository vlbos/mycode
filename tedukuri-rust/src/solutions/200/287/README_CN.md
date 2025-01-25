287\. 积蓄程度

*    [题目](https://www.acwing.com/problem/content/description/289/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/289/1/)
*    [题解](https://www.acwing.com/problem/content/solution/289/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/289/)

  

有一个树形的水系，由 N−1N−1 条河道和 NN 个交叉点组成。

我们可以把交叉点看作树中的节点，编号为 1∼N1∼N，河道则看作树中的无向边。

每条河道都有一个容量，连接 xx 与 yy 的河道的容量记为 c(x,y)c(x,y)。

河道中单位时间流过的水量不能超过河道的容量。

有一个节点是整个水系的发源地，可以源源不断地流出水，我们称之为源点。

除了源点之外，树中所有度数为 11 的节点都是入海口，可以吸收无限多的水，我们称之为汇点。

也就是说，水系中的水从源点出发，沿着每条河道，最终流向各个汇点。

在整个水系稳定时，每条河道中的水都以单位时间固定的水量流向固定的方向。

除源点和汇点之外，其余各点不贮存水，也就是流入该点的河道水量之和等于从该点流出的河道水量之和。

整个水系的流量就定义为源点单位时间发出的水量。

在流量不超过河道容量的前提下，求哪个点作为源点时，整个水系的流量最大，输出这个最大值。

#### 输入格式

输入第一行包含整数 TT，表示共有 TT 组测试数据。

每组测试数据，第一行包含整数 NN。

接下来 N−1N−1 行，每行包含三个整数 x,y,zx,y,z，表示 x，yx，y 之间存在河道，且河道容量为 zz。

节点编号从 11 开始。

#### 输出格式

每组数据输出一个结果，每个结果占一行。

数据保证结果不超过 231−1231−1。

#### 数据范围

N≤2×105N≤2×105

#### 输入样例：

    1
    5
    1 2 11
    1 4 13
    3 4 5
    4 5 10
    

#### 输出样例：

    26
    

难度：困难

时/空限制：1s / 64MB

总通过数：3433

总尝试数：9162

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3838&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3838&show_algorithm_tags=1)[树形DP](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E5%BD%A2DP&source_file_id=3838&show_algorithm_tags=1)[换根DP](https://www.acwing.com/problem/search/1/?search_content=%E6%8D%A2%E6%A0%B9DP&source_file_id=3838&show_algorithm_tags=1)