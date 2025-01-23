172\. 立体推箱子

*    [题目](https://www.acwing.com/problem/content/description/174/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/174/1/)
*    [题解](https://www.acwing.com/problem/content/solution/174/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/174/)

  

立体推箱子是一个风靡世界的小游戏。

游戏地图是一个 NN 行 MM 列的矩阵，每个位置可能是硬地（用 `.` 表示）、易碎地面（用 `E` 表示）、禁地（用 `#` 表示）、起点（用 `X` 表示）或终点（用 `O` 表示）。

你的任务是操作一个 1×1×21×1×2 的长方体。

这个长方体在地面上有两种放置形式，“立”在地面上（1×11×1 的面接触地面）或者“躺”在地面上（1×21×2 的面接触地面）。

在每一步操作中，可以按上下左右四个键之一。

按下按键之后，长方体向对应的方向沿着棱滚动 9090 度。

任意时刻，长方体不能有任何部位接触禁地，并且不能立在易碎地面上。

字符 `X` 标识长方体的起始位置，地图上可能有一个 `X` 或者两个相邻的 `X`。

地图上唯一的一个字符 `O` 标识目标位置。

求把长方体移动到目标位置（即立在 `O` 上）所需要的最少步数。

在移动过程中，`X` 和 `O` 标识的位置都可以看作是硬地被利用。

#### 输入格式

输入包含多组测试用例。

对于每个测试用例，第一行包括两个整数 NN 和 MM。

接下来 NN 行用来描述地图，每行包括 MM 个字符，每个字符表示一块地面的具体状态。

当输入用例 N\=0，M\=0N\=0，M\=0 时，表示输入终止，且该用例无需考虑。

#### 输出格式

每个用例输出一个整数表示所需的最少步数，如果无解则输出 `Impossible`。

每个结果占一行。

#### 数据范围

3≤N,M≤5003≤N,M≤500

#### 输入样例：

    7 7
    #######
    #..X###
    #..##O#
    #....E#
    #....E#
    #.....#
    #######
    0 0
    

#### 输出样例：

    10
    

难度：困难

时/空限制：1s / 64MB

总通过数：2563

总尝试数：7061

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3723&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3723&show_algorithm_tags=1)[广度优先搜索](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%BF%E5%BA%A6%E4%BC%98%E5%85%88%E6%90%9C%E7%B4%A2&source_file_id=3723&show_algorithm_tags=1)[BFS](https://www.acwing.com/problem/search/1/?search_content=BFS&source_file_id=3723&show_algorithm_tags=1)