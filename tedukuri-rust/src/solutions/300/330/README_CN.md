330\. 估算

*    [题目](https://www.acwing.com/problem/content/description/332/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/332/1/)
*    [题解](https://www.acwing.com/problem/content/solution/332/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/332/)

  

给定一个长度为 NN 的整数数组 AA，你需要创建另一个长度为 NN 的整数数组 BB，数组 BB 被分为 KK 个连续的部分，并且如果 ii 和 jj 在同一个部分，则 B\[i\]\=B\[j\]B\[i\]\=B\[j\]。

如果要求数组 BB 能够满足 Σ|A\[i\]−B\[i\]|Σ|A\[i\]−B\[i\]| 最小，那么最小值是多少，请你输出这个最小值。

#### 输入格式

输入包含不超过 2525 组测试数据。

对于每组测试数据，第一行包含两个整数 NN 和 KK。

接下来 NN 行每行包含一个整数，表示完整的数组 AA。

当输入为一行 `0 0` 时，表示输入终止。

#### 输出格式

对于每组数据，输出一个最小值。

每个结果占一行。

#### 数据范围

1≤N≤20001≤N≤2000,  
1≤K≤25,K≤N1≤K≤25,K≤N  
数组 AA 中元素的绝对值不超过 1000010000。

#### 输入样例：

    7 2
    6
    5
    4
    3
    2
    1
    7
    0 0
    

#### 输出样例：

    9
    

难度：中等

时/空限制：32s / 64MB

总通过数：510

总尝试数：1540

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3881&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3881&show_algorithm_tags=1)[堆优化DP](https://www.acwing.com/problem/search/1/?search_content=%E5%A0%86%E4%BC%98%E5%8C%96DP&source_file_id=3881&show_algorithm_tags=1)[中位数](https://www.acwing.com/problem/search/1/?search_content=%E4%B8%AD%E4%BD%8D%E6%95%B0&source_file_id=3881&show_algorithm_tags=1)