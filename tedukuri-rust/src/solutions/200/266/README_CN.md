266\. 超级备忘录

*    [题目](https://www.acwing.com/problem/content/description/268/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/268/1/)
*    [题解](https://www.acwing.com/problem/content/solution/268/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/268/)

  

你的朋友达达被邀请参加一个叫做“超级备忘录”的电视节目。

在这个节目中，参与者需要玩一个记忆游戏。

在一开始，主持人会告诉所有参与者一个数列，A1,A2,…,AnA1,A2,…,An。

接下来，主持人会在数列上做一些操作，操作包括以下几种：

1.  `ADD x y D`：给子序列 {Ax,…,Ay}{Ax,…,Ay} 统一加上一个数 DD。例如，在 {1,2,3,4,5}{1,2,3,4,5} 上进行操作 `ADD 2 4 1` 会得到 {1,3,4,5,5}{1,3,4,5,5}。
2.  `REVERSE x y`：将子序列 {Ax,…,Ay}{Ax,…,Ay} 逆序排布。例如，在 {1,2,3,4,5}{1,2,3,4,5} 上进行操作 `REVERSE 2 4` 会得到 {1,4,3,2,5}{1,4,3,2,5}。
3.  `REVOLVE x y T`：将子序列 {Ax,…,Ay}{Ax,…,Ay} 轮换 TT 次。例如，在 {1,2,3,4,5}{1,2,3,4,5} 上进行操作 `REVOLVE 2 4 2` 会得到 {1,3,4,2,5}{1,3,4,2,5}。
4.  `INSERT x P`：在 AxAx 后面插入 PP。例如，在 {1,2,3,4,5}{1,2,3,4,5} 上进行操作 `INSERT 2 4` 会得到 {1,2,4,3,4,5}{1,2,4,3,4,5}。
5.  `DELETE x`：删除 AxAx。例如，在 {1,2,3,4,5}{1,2,3,4,5} 上进行操作 `DELETE 2` 会得到 {1,3,4,5}{1,3,4,5}。
6.  `MIN x y`：询问子序列 {Ax,…,Ay}{Ax,…,Ay} 中的最小值。例如，{1,2,3,4,5}{1,2,3,4,5} 上执行 `MIN 2 4` 的正确答案为 22。

为了使得节目更加好看，每个参赛人都有机会在觉得困难时打电话请求场外观众的帮助。

你的任务是看这个电视节目，然后写一个程序对于每一个询问计算出结果，这样可以使得达达在任何时候打电话求助你的时候，你都可以给出正确答案。

#### 输入格式

第一行包含一个整数 nn。

接下来 nn 行给出了序列中的数。

接下来一行包含一个整数 MM，描述操作和询问的数量。

接下来 MM 行给出了所有的操作和询问。

#### 输出格式

对于每一个 `MIN` 询问，输出正确答案。

每个答案占一行。

#### 数据范围

n≤100000,M≤100000n≤100000,M≤100000,  
初始序列中数的范围 \[1,105\]\[1,105\]。  
操作保证合法，序列中的数始终都在 int 范围内。

#### 输入样例：

    5
    1 
    2 
    3 
    4 
    5
    2
    ADD 2 4 1
    MIN 4 5
    

#### 输出样例：

    5
    

难度：困难

时/空限制：2s / 64MB

总通过数：756

总尝试数：2009

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3817&show_algorithm_tags=0)[POJ3580](https://www.acwing.com/problem/search/1/?search_content=POJ3580&source_file_id=3817&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3817&show_algorithm_tags=0)

算法标签

[平衡树](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%B3%E8%A1%A1%E6%A0%91&source_file_id=3817&show_algorithm_tags=1)[Splay](https://www.acwing.com/problem/search/1/?search_content=Splay&source_file_id=3817&show_algorithm_tags=1)