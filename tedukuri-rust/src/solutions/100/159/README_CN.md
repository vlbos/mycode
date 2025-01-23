159\. 奶牛矩阵

*    [题目](https://www.acwing.com/problem/content/description/161/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/161/1/)
*    [题解](https://www.acwing.com/problem/content/solution/161/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/161/)

  

每天早上，农夫约翰的奶牛们被挤奶的时候，都会站成一个 RR 行 CC 列的方阵。

现在在每个奶牛的身上标注表示其品种的大写字母，则所有奶牛共同构成了一个 RR 行 CC 列的字符矩阵。

现在给定由所有奶牛构成的矩阵，求它的最小覆盖子矩阵的面积是多少。

如果一个子矩阵无限复制扩张之后得到的矩阵能包含原来的矩阵，则称该子矩阵为覆盖子矩阵。

#### 输入格式

第 11 行：输入两个用空格隔开的整数，RR 和 CC。

第 2..R+12..R+1 行：描绘由奶牛构成的 RR 行 CC 列的矩阵，每行 CC 个字符，字符之间没有空格。

#### 输出格式

输出最小覆盖子矩阵的面积。（每个字符的面积为 11）

#### 数据范围

1≤R≤100001≤R≤10000,  
1≤C≤751≤C≤75

#### 输入样例：

    2 5
    ABABA
    ABABA
    

#### 输出样例：

    2
    

#### 提示

样例中给出的矩阵的最小覆盖子矩阵为 ABAB，面积为 22。

难度：中等

时/空限制：1s / 64MB

总通过数：1984

总尝试数：4650

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3710&show_algorithm_tags=0)

算法标签

[字符串](https://www.acwing.com/problem/search/1/?search_content=%E5%AD%97%E7%AC%A6%E4%B8%B2&source_file_id=3710&show_algorithm_tags=1)[KMP模式匹配](https://www.acwing.com/problem/search/1/?search_content=KMP%E6%A8%A1%E5%BC%8F%E5%8C%B9%E9%85%8D&source_file_id=3710&show_algorithm_tags=1)