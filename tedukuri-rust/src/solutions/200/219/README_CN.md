219\. 剪纸游戏

*    [题目](https://www.acwing.com/problem/content/description/221/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/221/1/)
*    [题解](https://www.acwing.com/problem/content/solution/221/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/221/)

  

给定一张 N×MN×M 的矩形网格纸，两名玩家轮流行动。

在每一次行动中，可以任选一张矩形网格纸，沿着某一行或某一列的格线，把它剪成两部分。

首先剪出 1×11×1 的格纸的玩家获胜。

两名玩家都采取最优策略行动，求先手是否能获胜。

提示：开始时只有一张纸可以进行裁剪，随着游戏进行，纸张被裁剪成 2,3,…2,3,… 更多张，可选择进行裁剪的纸张就会越来越多。

#### 输入格式

输入包含多组测试数据，每组数据占一行。

每组数据包括两个整数 NN 和 MM，表示初始网格纸的尺寸。

#### 输出格式

每组测试数据输出一个结果，结果占一行。

如果先手方必胜，则输出 `WIN`；

如果先手方必输，则输出 `LOSE`。

#### 数据范围

2≤N,M≤2002≤N,M≤200

#### 输入样例：

    2 2
    3 2
    4 2
    

#### 输出样例：

    LOSE
    LOSE
    WIN
    

难度：中等

时/空限制：1s / 64MB

总通过数：1444

总尝试数：3191

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3770&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3770&show_algorithm_tags=1)[博弈论](https://www.acwing.com/problem/search/1/?search_content=%E5%8D%9A%E5%BC%88%E8%AE%BA&source_file_id=3770&show_algorithm_tags=1)[SG函数](https://www.acwing.com/problem/search/1/?search_content=SG%E5%87%BD%E6%95%B0&source_file_id=3770&show_algorithm_tags=1)