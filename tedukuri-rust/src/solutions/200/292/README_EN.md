292\. 炮兵阵地

*    [题目](https://www.acwing.com/problem/content/description/294/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/294/1/)
*    [题解](https://www.acwing.com/problem/content/solution/294/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/294/)

  

司令部的将军们打算在 N×MN×M 的网格地图上部署他们的炮兵部队。

一个 N×MN×M 的地图由 NN 行 MM 列组成，地图的每一格可能是山地（用 `H` 表示），也可能是平原（用 `P` 表示），如下图。

在每一格平原地形上最多可以布置一支炮兵部队（山地上不能够部署炮兵部队）；一支炮兵部队在地图上的攻击范围如图中黑色区域所示：

![1185_1.jpg](https://cdn.acwing.com/media/article/image/2019/02/16/19_d512cdba31-1185_1.jpg)

如果在地图中的灰色所标识的平原上部署一支炮兵部队，则图中的黑色的网格表示它能够攻击到的区域：沿横向左右各两格，沿纵向上下各两格。

图上其它白色网格均攻击不到。

从图上可见炮兵的攻击范围不受地形的影响。

现在，将军们规划如何部署炮兵部队，在防止误伤的前提下（保证任何两支炮兵部队之间不能互相攻击，即任何一支炮兵部队都不在其他支炮兵部队的攻击范围内），在整个地图区域内最多能够摆放多少我军的炮兵部队。

#### 输入格式

第一行包含两个由空格分割开的正整数，分别表示 NN 和 MM；

接下来的 NN 行，每一行含有连续的 MM 个字符(`P` 或者 `H`)，中间没有空格。按顺序表示地图中每一行的数据。

#### 输出格式

仅一行，包含一个整数 KK，表示最多能摆放的炮兵部队的数量。

#### 数据范围

N≤100,M≤10N≤100,M≤10

#### 输入样例：

    5 4
    PHPP
    PPHH
    PPPP
    PHPP
    PHHP
    

#### 输出样例：

    6
    

难度：中等

时/空限制：1s / 64MB

总通过数：18552

总尝试数：35186

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3843&show_algorithm_tags=0)[NOI2001](https://www.acwing.com/problem/search/1/?search_content=NOI2001&source_file_id=3843&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3843&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3843&show_algorithm_tags=1)[状态压缩DP](https://www.acwing.com/problem/search/1/?search_content=%E7%8A%B6%E6%80%81%E5%8E%8B%E7%BC%A9DP&source_file_id=3843&show_algorithm_tags=1)