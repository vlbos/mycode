375\. 蚂蚁

*    [题目](https://www.acwing.com/problem/content/description/377/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/377/1/)
*    [题解](https://www.acwing.com/problem/content/solution/377/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/377/)

  

平面上共有 2×N2×N 个点，NN 个是白点，NN 个是黑点。

对于每个白点，找到一个黑点，把二者用线连起来，要求最后所有线段都不相交，求一种方案。

#### 输入格式

第一行包含整数 NN。

接下来 NN 行，每行两个整数，表示一个黑点的坐标。

再接下来 NN 行，每行两个整数，表示一个白点的坐标。

#### 输出格式

输出共 NN 行，每行一个整数。

第 ii 行的数，表示第 ii 个黑点连接的白点的编号，编号从 11 开始。

注意答案可能不唯一，任意输出一种答案即可。

#### 数据范围

1≤N≤1001≤N≤100,坐标绝对值不超过 1000010000。

#### 输入样例：

    5
    -42 58
    44 86
    7 28
    99 34
    -13 -59
    -47 -44
    86 74
    68 -75
    -68 60
    99 -60
    

#### 输出样例：

    4
    2
    1
    5
    3
    

难度：困难

时/空限制：1s / 64MB

总通过数：777

总尝试数：1980

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3926&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3926&show_algorithm_tags=1)[二分图带权匹配](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86%E5%9B%BE%E5%B8%A6%E6%9D%83%E5%8C%B9%E9%85%8D&source_file_id=3926&show_algorithm_tags=1)