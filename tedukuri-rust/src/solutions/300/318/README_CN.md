318\. 划分大理石

*    [题目](https://www.acwing.com/problem/content/description/320/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/320/1/)
*    [题解](https://www.acwing.com/problem/content/solution/320/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/320/)

  

有价值分别为 1..61..6 的大理石各 a\[1..6\]a\[1..6\] 块，现要将它们分成两部分，使得两部分价值之和相等，问是否可以实现。

其中大理石的总数不超过 2000020000。

#### 输入格式

输入包含多组数据！

每组数据占一行，包含 66 个整数，表示 a\[1\]∼a\[6\]a\[1\]∼a\[6\]。

当输入为 `0 0 0 0 0 0` 时表示输入结束，且该行无需考虑。

#### 输出格式

每组数据输出一个结果，每个结果占一行。

如果可以实现则输出 `Can`，否则输出 `Can't`。

#### 输入样例：

    4 7 4 5 9 1
    9 8 1 7 2 4
    6 6 8 5 9 2
    1 6 6 1 0 7
    5 9 3 8 8 4
    0 0 0 0 0 0
    

#### 输出样例：

    Can't
    Can
    Can't
    Can't
    Can
    

难度：中等

时/空限制：1s / 64MB

总通过数：1166

总尝试数：2667

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3869&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3869&show_algorithm_tags=1)[多重背包](https://www.acwing.com/problem/search/1/?search_content=%E5%A4%9A%E9%87%8D%E8%83%8C%E5%8C%85&source_file_id=3869&show_algorithm_tags=1)