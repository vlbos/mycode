121\. 赶牛入圈

*    [题目](https://www.acwing.com/problem/content/description/123/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/123/1/)
*    [题解](https://www.acwing.com/problem/content/solution/123/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/123/)

  

农夫约翰希望为他的奶牛们建立一个畜栏。

这些挑剔的畜生要求畜栏必须是正方形的，而且至少要包含 CC 单位的三叶草，来当做它们的下午茶。

畜栏的边缘必须与 X，YX，Y 轴平行。

约翰的土地里一共包含 NN 单位的三叶草，每单位三叶草位于一个 1×11×1 的土地区域内，区域位置由其左下角坐标表示，并且区域左下角的 X,YX,Y 坐标都为整数，范围在 11 到 1000010000 以内。

多个单位的三叶草可能会位于同一个 1×11×1 的区域内，因为这个原因，在接下来的输入中，同一个区域坐标可能出现多次。

只有一个区域完全位于修好的畜栏之中，才认为这个区域内的三叶草在畜栏之中。

请你帮约翰计算一下，能包含至少 CC 单位面积三叶草的情况下，畜栏的最小边长是多少。

#### 输入格式

第一行输入两个整数 CC 和 NN。

接下来 NN 行，每行输入两个整数 XX 和 YY，代表三叶草所在的区域的 X,YX,Y 坐标。

同一行数据用空格隔开。

#### 输出格式

输出一个整数，代表畜栏的最小边长。

#### 数据范围

1≤C≤5001≤C≤500,  
C≤N≤500C≤N≤500

#### 输入样例：

    3 4
    1 2
    2 1
    4 1
    5 2
    

#### 输出样例：

    4
    

难度：中等

时/空限制：1s / 64MB

总通过数：3292

总尝试数：9394

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3672&show_algorithm_tags=0)

算法标签

[二分](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86&source_file_id=3672&show_algorithm_tags=1)[前缀和](https://www.acwing.com/problem/search/1/?search_content=%E5%89%8D%E7%BC%80%E5%92%8C&source_file_id=3672&show_algorithm_tags=1)[离散化](https://www.acwing.com/problem/search/1/?search_content=%E7%A6%BB%E6%95%A3%E5%8C%96&source_file_id=3672&show_algorithm_tags=1)