327\. 玉米田

*    [题目](https://www.acwing.com/problem/content/description/329/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/329/1/)
*    [题解](https://www.acwing.com/problem/content/solution/329/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/329/)

  

农夫约翰的土地由 M×NM×N 个小方格组成，现在他要在土地里种植玉米。

非常遗憾，部分土地是不育的，无法种植。

而且，相邻的土地不能同时种植玉米，也就是说种植玉米的所有方格之间都不会有公共边缘。

现在给定土地的大小，请你求出共有多少种种植方法。

土地上什么都不种也算一种方法。

#### 输入格式

第 11 行包含两个整数 MM 和 NN。

第 2..M+12..M+1 行：每行包含 NN 个整数 00 或 11，用来描述整个土地的状况，11 表示该块土地肥沃，00 表示该块土地不育。

#### 输出格式

输出总种植方法对 108108 取模后的值。

#### 数据范围

1≤M,N≤121≤M,N≤12

#### 输入样例：

    2 3
    1 1 1
    0 1 0
    

#### 输出样例：

    9
    

难度：简单

时/空限制：1s / 64MB

总通过数：16566

总尝试数：27468

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3878&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3878&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3878&show_algorithm_tags=1)[状态压缩DP](https://www.acwing.com/problem/search/1/?search_content=%E7%8A%B6%E6%80%81%E5%8E%8B%E7%BC%A9DP&source_file_id=3878&show_algorithm_tags=1)