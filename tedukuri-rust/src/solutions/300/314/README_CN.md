314\. 低买

*    [题目](https://www.acwing.com/problem/content/description/316/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/316/1/)
*    [题解](https://www.acwing.com/problem/content/solution/316/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/316/)

  

给定一段时间内股票的每日售价（正 1616 位整数）。

你可以选择在任何一天购买股票。

每次你选择购买时，当前的股票价格必须严格低于你之前购买股票时的价格。

编写一个程序，确定你应该在哪些天购进股票，可以使得你能够购买股票的次数最大化。

例如，下面是一个股票价格时间表：

     Day   1  2  3  4  5  6  7  8  9 10 11 12
    
    Price 68 69 54 64 68 64 70 67 78 62 98 87
    

如果每次购买都必须遵循当前股票价格严格低于之前购买股票时的价格，那么投资者最多可以购买四次该股票。

买进方案之一为：

    Day    2  5  6 10
    
    Price 69 68 64 62
    

#### 输入格式

第 11 行包含整数 NN，表示给出的股票价格的天数。

第 22 至最后一行，共包含 NN 个整数，每行 1010 个，最后一行可能不够 1010 个，表示 NN 天的股票价格。

同一行数之间用空格隔开。

#### 输出格式

输出占一行，包含两个整数，分别表示最大买进股票次数以及可以达到最大买进次数的方案数。

如果两种方案的买入日序列不同，但是价格序列相同，则认为这是相同的方案（只计算一次）。

#### 数据范围

1≤N≤50001≤N≤5000,  
保证答案均不超过 intint 范围。

#### 输入样例1：

    12
    68 69 54 64 68 64 70 67 78 62
    98 87
    

#### 输出样例1：

    4 2
    

#### 输入样例2：

    5
    4 3 2 1 1
    

#### 输出样例2：

    4 1
    

难度：中等

时/空限制：1s / 30MB

总通过数：1727

总尝试数：3562

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3865&show_algorithm_tags=0)[usaco training 4.3](https://www.acwing.com/problem/search/1/?search_content=usaco%20training%204.3&source_file_id=3865&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3865&show_algorithm_tags=1)[线性DP](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7DP&source_file_id=3865&show_algorithm_tags=1)[统计LIS方案数](https://www.acwing.com/problem/search/1/?search_content=%E7%BB%9F%E8%AE%A1LIS%E6%96%B9%E6%A1%88%E6%95%B0&source_file_id=3865&show_algorithm_tags=1)