1055\. 股票买卖 II

*    [题目](https://www.acwing.com/problem/content/description/1057/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/1057/1/)
*    [题解](https://www.acwing.com/problem/content/solution/1057/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/1057/)

  

给定一个长度为 NN 的数组，数组中的第 ii 个数字表示一个给定股票在第 ii 天的价格。

设计一个算法来计算你所能获取的最大利润。你可以尽可能地完成更多的交易（多次买卖一支股票）。

注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

#### 输入格式

第一行包含整数 NN，表示数组长度。

第二行包含 NN 个不大于 1000010000 的正整数，表示完整的数组。

#### 输出格式

输出一个整数，表示最大利润。

#### 数据范围

1≤N≤1051≤N≤105

#### 输入样例1：

    6
    7 1 5 3 6 4
    

#### 输出样例1：

    7
    

#### 输入样例2：

    5
    1 2 3 4 5
    

#### 输出样例2：

    4
    

#### 输入样例3：

    5
    7 6 4 3 1
    

#### 输出样例3：

    0
    

#### 样例解释

样例1：在第 2 天（股票价格 = 1）的时候买入，在第 3 天（股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。随后，在第 4 天（股票价格 = 3）的时候买入，在第 5 天（股票价格 = 6）的时候卖出, 这笔交易所能获得利润 = 6-3 = 3 。共得利润 4+3 = 7。

样例2：在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。

样例3：在这种情况下, 不进行任何交易, 所以最大利润为 0。

难度：简单

时/空限制：1s / 64MB

总通过数：14761

总尝试数：19919

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=4184000&show_algorithm_tags=0)[LeetCode](https://www.acwing.com/problem/search/1/?search_content=LeetCode&source_file_id=4184000&show_algorithm_tags=0)

算法标签

[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=4184000&show_algorithm_tags=1)[DP](https://www.acwing.com/problem/search/1/?search_content=DP&source_file_id=4184000&show_algorithm_tags=1)[线性DP](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7DP&source_file_id=4184000&show_algorithm_tags=1)[状态机](https://www.acwing.com/problem/search/1/?search_content=%E7%8A%B6%E6%80%81%E6%9C%BA&source_file_id=4184000&show_algorithm_tags=1)