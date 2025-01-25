334\. K匿名序列

*    [题目](https://www.acwing.com/problem/content/description/336/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/336/1/)
*    [题解](https://www.acwing.com/problem/content/solution/336/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/336/)

  

给出一个长度为 nn 的非严格递增整数序列，每次操作可以将其中的一个数减少一，问最少多少次操作后能够使得序列中的任何一个数在序列中都至少有 k−1k−1 个数与之相同。

#### 输入格式

第一行包含整数 TT，表示共有 TT 组测试数据。

每组测试数据，第一行包含两个整数 nn 和 kk。

第二行包含 nn 个不超过 500,000500,000 的非负整数，表示完整的整数序列。

#### 输出格式

每组测试数据输出一个整数，表示所需最少操作数。

每个结果占一行。

#### 数据范围

1≤T≤201≤T≤20,  
2≤n≤5000002≤n≤500000,  
2≤k≤n2≤k≤n

#### 输入样例：

    2
    7 3
    2 2 3 4 4 5 5
    6 2
    0 3 3 4 8 9
    

#### 输出样例：

    3
    5
    

难度：中等

时/空限制：1s / 64MB

总通过数：507

总尝试数：1376

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3885&show_algorithm_tags=0)[POJ3709](https://www.acwing.com/problem/search/1/?search_content=POJ3709&source_file_id=3885&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3885&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3885&show_algorithm_tags=1)[斜率优化](https://www.acwing.com/problem/search/1/?search_content=%E6%96%9C%E7%8E%87%E4%BC%98%E5%8C%96&source_file_id=3885&show_algorithm_tags=1)