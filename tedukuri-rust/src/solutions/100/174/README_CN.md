174\. 推箱子

*    [题目](https://www.acwing.com/problem/content/description/176/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/176/1/)
*    [题解](https://www.acwing.com/problem/content/solution/176/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/176/)

  

推箱子游戏相信大家都不陌生，在本题中，你将控制一个人把 11 个箱子到目的地。

给定一张 NN 行 MM 列的地图，用字符 `.` 表示空地，字符 `#` 表示墙，字符 `S` 表示人的起始位置，字符 `B` 表示箱子的起始位置，字符 `T` 表示箱子的目标位置。

求一种移动方案，使箱子移动的次数最少，在此基础上再让人移动的总步数最少。

方案中使用大写的 `EWSN`（东西南北）表示箱子的移动，使用小写的 `ewsn`（东西南北）表示人的移动。

![推箱子.jpg](https://cdn.acwing.com/media/article/image/2019/01/16/19_8c8e5b0a19-%E6%8E%A8%E7%AE%B1%E5%AD%90.jpg)

#### 输入格式

输入包含多个测试用例。

对于每个测试用例，第一行包括两个整数 N，MN，M。

接下来 NN 行，每行包括 MM 个字符，用以描绘整个 NN 行 MM 列的地图。

当样例为 N\=0，M\=0N\=0，M\=0 时，表示输入终止，且该样例无需处理。

#### 输出格式

对于每个测试用例，第一行输出 `Maze #`+测试用例的序号。

第二行输入一个字符串，表示推箱子的总体移动过程，若无解，则输出 `Impossible.`。

每个测试用例输出结束后输出一个空行。

若有多条路线满足题目要求，则按照 `N`、`S`、`W`、`E` 的顺序优先选择箱子的移动方向（即先上下推，再左右推）。

在此前提下，再按照 `n`、`s`、`w`、`e` 的顺序优先选择人的移动方向（即先上下动，再左右动）。

#### 数据范围

1≤N,M≤201≤N,M≤20

#### 输入样例：

    1 7
    SB....T
    1 7
    SB..#.T
    7 11
    ###########
    #T##......#
    #.#.#..####
    #....B....#
    #.######..#
    #.....S...#
    ###########
    8 4
    ....
    .##.
    .#..
    .#..
    .#.B
    .##S
    ....
    ###T
    0 0
    

#### 输出样例：

    Maze #1
    EEEEE
    
    Maze #2
    Impossible.
    
    Maze #3
    eennwwWWWWeeeeeesswwwwwwwnNN
    
    Maze #4
    swwwnnnnnneeesssSSS
    
    

难度：困难

时/空限制：1s / 64MB

总通过数：1578

总尝试数：5187

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3725&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3725&show_algorithm_tags=1)[广度优先搜索](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%BF%E5%BA%A6%E4%BC%98%E5%85%88%E6%90%9C%E7%B4%A2&source_file_id=3725&show_algorithm_tags=1)[BFS](https://www.acwing.com/problem/search/1/?search_content=BFS&source_file_id=3725&show_algorithm_tags=1)