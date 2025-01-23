123\. 士兵

*    [题目](https://www.acwing.com/problem/content/description/125/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/125/1/)
*    [题解](https://www.acwing.com/problem/content/solution/125/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/125/)

  

格格兰郡的 NN 名士兵随机散落在全郡各地。

格格兰郡中的位置由一对 (x,y)(x,y) 整数坐标表示。

士兵可以进行移动，每次移动，一名士兵可以向上，向下，向左或向右移动一个单位（因此，他的 xx 或 yy 坐标也将加 11 或减 11）。

现在希望通过移动士兵，使得所有士兵彼此相邻的处于同一条水平线内，即所有士兵的 yy 坐标相同并且 xx 坐标相邻。

请你计算满足要求的情况下，所有士兵的总移动次数最少是多少。

需注意，两个或多个士兵不能占据同一个位置。

#### 输入格式

第一行输入整数 NN，代表士兵的数量。

接下来的 NN 行，每行输入两个整数 xx 和 yy，分别代表一个士兵所在位置的 xx 坐标和 yy 坐标，第 ii 行即为第 ii 个士兵的坐标 (x\[i\],y\[i\])(x\[i\],y\[i\])。

#### 输出格式

输出一个整数，代表所有士兵的总移动次数的最小值。

#### 数据范围

1≤N≤100001≤N≤10000,  
−10000≤x\[i\],y\[i\]≤10000−10000≤x\[i\],y\[i\]≤10000

#### 输入样例：

    5
    1 2
    2 2
    1 3
    3 -2
    3 3
    

#### 输出样例：

    8
    

难度：中等

时/空限制：1s / 64MB

总通过数：2802

总尝试数：5237

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3674&show_algorithm_tags=0)

算法标签

[排序](https://www.acwing.com/problem/search/1/?search_content=%E6%8E%92%E5%BA%8F&source_file_id=3674&show_algorithm_tags=1)[绝对值不等式](https://www.acwing.com/problem/search/1/?search_content=%E7%BB%9D%E5%AF%B9%E5%80%BC%E4%B8%8D%E7%AD%89%E5%BC%8F&source_file_id=3674&show_algorithm_tags=1)