255\. 第K小数

*    [题目](https://www.acwing.com/problem/content/description/257/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/257/1/)
*    [题解](https://www.acwing.com/problem/content/solution/257/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/257/)

  

给定长度为 NN 的整数序列 AA，下标为 1∼N1∼N。

现在要执行 MM 次操作，其中第 ii 次操作为给出三个整数 li,ri,kili,ri,ki，求 A\[li\],A\[li+1\],…,A\[ri\]A\[li\],A\[li+1\],…,A\[ri\] (即 AA 的下标区间 \[li,ri\]\[li,ri\])中第 kiki 小的数是多少。

#### 输入格式

第一行包含两个整数 NN 和 MM。

第二行包含 NN 个整数，表示整数序列 AA。

接下来 MM 行，每行包含三个整数 li,ri,kili,ri,ki，用以描述第 ii 次操作。

#### 输出格式

对于每次操作输出一个结果，表示在该次操作中，第 kk 小的数的数值。

每个结果占一行。

#### 数据范围

N≤105,M≤104,|A\[i\]|≤109N≤105,M≤104,|A\[i\]|≤109

#### 输入样例：

    7 3
    1 5 2 6 3 7 4
    2 5 3
    4 4 1
    1 7 3
    

#### 输出样例：

    5
    6
    3
    

难度：中等

时/空限制：1s / 64MB

总通过数：9240

总尝试数：14918

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3806&show_algorithm_tags=0)[POJ2104](https://www.acwing.com/problem/search/1/?search_content=POJ2104&source_file_id=3806&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3806&show_algorithm_tags=0)

算法标签

[离线分治](https://www.acwing.com/problem/search/1/?search_content=%E7%A6%BB%E7%BA%BF%E5%88%86%E6%B2%BB&source_file_id=3806&show_algorithm_tags=1)[基于值域的整体分治算法](https://www.acwing.com/problem/search/1/?search_content=%E5%9F%BA%E4%BA%8E%E5%80%BC%E5%9F%9F%E7%9A%84%E6%95%B4%E4%BD%93%E5%88%86%E6%B2%BB%E7%AE%97%E6%B3%95&source_file_id=3806&show_algorithm_tags=1)[可持久化线段树](https://www.acwing.com/problem/search/1/?search_content=%E5%8F%AF%E6%8C%81%E4%B9%85%E5%8C%96%E7%BA%BF%E6%AE%B5%E6%A0%91&source_file_id=3806&show_algorithm_tags=1)