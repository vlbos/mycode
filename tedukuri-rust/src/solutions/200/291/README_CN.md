291\. 蒙德里安的梦想

*    [题目](https://www.acwing.com/problem/content/description/293/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/293/1/)
*    [题解](https://www.acwing.com/problem/content/solution/293/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/293/)

  

求把 N×MN×M 的棋盘分割成若干个 1×21×2 的长方形，有多少种方案。

例如当 N\=2，M\=4N\=2，M\=4 时，共有 55 种方案。当 N\=2，M\=3N\=2，M\=3 时，共有 33 种方案。

如下图所示：

![2411_1.jpg](https://cdn.acwing.com/media/article/image/2019/01/26/19_4dd1644c20-2411_1.jpg)

#### 输入格式

输入包含多组测试用例。

每组测试用例占一行，包含两个整数 NN 和 MM。

当输入用例 N\=0，M\=0N\=0，M\=0 时，表示输入终止，且该用例无需处理。

#### 输出格式

每个测试用例输出一个结果，每个结果占一行。

#### 数据范围

1≤N,M≤111≤N,M≤11

#### 输入样例：

    1 2
    1 3
    1 4
    2 2
    2 3
    2 4
    2 11
    4 11
    0 0
    

#### 输出样例：

    1
    0
    1
    2
    3
    5
    144
    51205
    

难度：中等

时/空限制：1.5s / 64MB

总通过数：48412

总尝试数：85496

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3842&show_algorithm_tags=0)[模板题](https://www.acwing.com/problem/search/1/?search_content=%E6%A8%A1%E6%9D%BF%E9%A2%98&source_file_id=3842&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3842&show_algorithm_tags=1)[状态压缩DP](https://www.acwing.com/problem/search/1/?search_content=%E7%8A%B6%E6%80%81%E5%8E%8B%E7%BC%A9DP&source_file_id=3842&show_algorithm_tags=1)