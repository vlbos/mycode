288\. 休息时间

*    [题目](https://www.acwing.com/problem/content/description/290/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/290/1/)
*    [题解](https://www.acwing.com/problem/content/solution/290/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/290/)

  

在某个星球上，一天由 NN 个小时构成，我们称 00 点到 11 点为第 11 个小时、11 点到 22 点为第 22 个小时，以此类推。

在第 ii 个小时睡觉能够恢复 UiUi 点体力。

在这个星球上住着一头牛，它每天要休息 BB 个小时。

它休息的这 BB 个小时不一定连续，可以分成若干段，但是在每段的第一个小时，它需要从清醒逐渐入睡，不能恢复体力，从下一个小时开始才能睡着。

为了身体健康，这头牛希望遵循生物钟，每天采用相同的睡觉计划。

另外，因为时间是连续的，即每一天的第 NN 个小时和下一天的第 11 个小时是相连的（NN 点等于 00 点），这头牛只需要在每 NN 个小时内休息够 BB 个小时就可以了。

请你帮忙给这头牛安排一个睡觉计划，使它每天恢复的体力最多。

#### 输入格式

第 11 行输入两个空格隔开的整数 NN 和 BB。

第 2..N+12..N+1 行，第 i+1i+1 行包含一个整数 UiUi。

#### 输出格式

输出一个整数，表示恢复的体力值。

#### 数据范围

3≤N≤38303≤N≤3830  
2≤B<N2≤B<N  
0≤Ui≤2000000≤Ui≤200000

#### 输入样例：

    5 3
    2
    0
    3
    1
    4
    

#### 输出样例：

    6
    

#### 样例解释

这头牛每天 33 点入睡，睡到次日 11 点，即 \[1,4,2\]\[1,4,2\] 时间段休息，每天恢复体力值最大，为 0+4+2\=60+4+2\=6。

难度：中等

时/空限制：1s / 64MB

总通过数：2219

总尝试数：5862

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3839&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3839&show_algorithm_tags=1)[环形结构](https://www.acwing.com/problem/search/1/?search_content=%E7%8E%AF%E5%BD%A2%E7%BB%93%E6%9E%84&source_file_id=3839&show_algorithm_tags=1)