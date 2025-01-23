180\. 排书

*    [题目](https://www.acwing.com/problem/content/description/182/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/182/1/)
*    [题解](https://www.acwing.com/problem/content/solution/182/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/182/)

  

给定 nn 本书，编号为 1∼n1∼n。

在初始状态下，书是任意排列的。

在每一次操作中，可以抽取其中连续的一段，再把这段插入到其他某个位置。

我们的目标状态是把书按照 1∼n1∼n 的顺序依次排列。

求最少需要多少次操作。

#### 输入格式

第一行包含整数 TT，表示共有 TT 组测试数据。

每组数据包含两行，第一行为整数 nn，表示书的数量。

第二行为 nn 个整数，表示 1∼n1∼n 的一种任意排列。

同行数之间用空格隔开。

#### 输出格式

每组数据输出一个最少操作次数。

如果最少操作次数大于或等于 55 次，则输出 `5 or more`。

每个结果占一行。

#### 数据范围

1≤n≤151≤n≤15

#### 输入样例：

    3
    6
    1 3 4 6 2 5
    5
    5 4 3 2 1
    10
    6 8 5 3 4 7 2 9 1 10
    

#### 输出样例：

    2
    3
    5 or more
    

难度：中等

时/空限制：1s / 64MB

总通过数：7955

总尝试数：13179

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3731&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3731&show_algorithm_tags=1)[IDA\*](https://www.acwing.com/problem/search/1/?search_content=IDA*&source_file_id=3731&show_algorithm_tags=1)