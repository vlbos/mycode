107\. 超快速排序

*    [题目](https://www.acwing.com/problem/content/description/109/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/109/1/)
*    [题解](https://www.acwing.com/problem/content/solution/109/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/109/)

  

在这个问题中，您必须分析特定的排序算法----超快速排序。

该算法通过交换两个相邻的序列元素来处理 nn 个不同整数的序列，直到序列按升序排序。

对于输入序列 `9 1 0 5 4`，超快速排序生成输出 `0 1 4 5 9`。

您的任务是确定超快速排序需要执行多少交换操作才能对给定的输入序列进行排序。

#### 输入格式

输入包括一些测试用例。

每个测试用例的第一行输入整数 nn，代表该用例中输入序列的长度。

接下来 nn 行每行输入一个整数 aiai,代表用例中输入序列的具体数据，第 ii 行的数据代表序列中第 ii 个数。

当输入用例中包含的输入序列长度为 00 时，输入终止，该序列无需处理。

#### 输出格式

对于每个需要处理的输入序列，输出一个整数 opop，代表对给定输入序列进行排序所需的最小交换操作数，每个整数占一行。

#### 数据范围

0≤n<5000000≤n<500000,  
一个测试点中，所有 nn 的和不超过 500000500000。  
0≤ai≤9999999990≤ai≤999999999

#### 输入样例：

    5
    9
    1
    0
    5
    4
    3
    1
    2
    3
    0
    

#### 输出样例：

    6
    0
    

难度：简单

时/空限制：7s / 64MB

总通过数：13894

总尝试数：28764

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3658&show_algorithm_tags=0)

算法标签

[逆序对](https://www.acwing.com/problem/search/1/?search_content=%E9%80%86%E5%BA%8F%E5%AF%B9&source_file_id=3658&show_algorithm_tags=1)[归并排序](https://www.acwing.com/problem/search/1/?search_content=%E5%BD%92%E5%B9%B6%E6%8E%92%E5%BA%8F&source_file_id=3658&show_algorithm_tags=1)[树状数组](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E7%8A%B6%E6%95%B0%E7%BB%84&source_file_id=3658&show_algorithm_tags=1)