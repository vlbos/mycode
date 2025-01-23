194\. 涂满它！

*    [题目](https://www.acwing.com/problem/content/description/196/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/196/1/)
*    [题解](https://www.acwing.com/problem/content/solution/196/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/196/)

  

Flood-it 是谷歌+平台上的非常好玩的一款游戏，游戏界面如下所示：

![flood.png](https://cdn.acwing.com/media/article/image/2019/01/17/19_30d05cba19-flood.png)

在游戏开始时，系统将随机生成 N×NN×N 的方形区域，并且区域内的每个网格都被涂成了六种颜色中的一种。

玩家从左上角开始游戏。

在每个步骤中，玩家选择一种颜色并将与左上角连通的所有格子（包括左上角）都变成该种颜色。

这里连通定义为：两个格子有公共边，并且颜色相同。

通过这种方式，玩家可以从左上角开始将所有格子都变为同一种颜色。

下图显示了 4×44×4 游戏的最早步骤（颜色标记为 00 到 55）：

![2.png](https://cdn.acwing.com/media/article/image/2019/01/17/19_c741618419-2.png)

请你求出，给定最初区域以后，最少要多少步才能把所有格子的颜色变成一样的。

#### 输入格式

输入包含不超过 2020 个测试用例。

每个测试用例，第一行包含一个整数 NN，表示方形区域大小。

接下里 NN 行，每行包含 NN 个整数（0−50−5），第 ii 行第 jj 个整数表示第 ii 行第 jj 列的格子的颜色。

当输入样例 N\=0N\=0 时，表示输入终止，该用例无需处理。

#### 输出格式

每个测试用例输出一个占据一行的整数，表示所需最少步数。

#### 数据范围

2≤N≤82≤N≤8

#### 输入样例：

    2
    0 0 
    0 0
    3
    0 1 2
    1 1 2
    2 2 1
    0
    

#### 输出样例：

    0
    3
    

难度：中等

时/空限制：1s / 64MB

总通过数：766

总尝试数：2378

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3745&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3745&show_algorithm_tags=1)[IDA\*](https://www.acwing.com/problem/search/1/?search_content=IDA*&source_file_id=3745&show_algorithm_tags=1)