188\. 武士风度的牛

*    [题目](https://www.acwing.com/problem/content/description/190/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/190/1/)
*    [题解](https://www.acwing.com/problem/content/solution/190/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/190/)

  

农民 John 有很多牛，他想交易其中一头被 Don 称为 The Knight 的牛。

这头牛有一个独一无二的超能力，在农场里像 Knight 一样地跳（就是我们熟悉的象棋中马的走法）。

虽然这头神奇的牛不能跳到树上和石头上，但是它可以在牧场上随意跳，我们把牧场用一个 x，yx，y 的坐标图来表示。

这头神奇的牛像其它牛一样喜欢吃草，给你一张地图，上面标注了 The Knight 的开始位置，树、灌木、石头以及其它障碍的位置，除此之外还有一捆草。

现在你的任务是，确定 The Knight 要想吃到草，至少需要跳多少次。

The Knight 的位置用 `K` 来标记，障碍的位置用 `*` 来标记，草的位置用 `H` 来标记。

这里有一个地图的例子：

                 11 | . . . . . . . . . .
                 10 | . . . . * . . . . . 
                  9 | . . . . . . . . . . 
                  8 | . . . * . * . . . . 
                  7 | . . . . . . . * . . 
                  6 | . . * . . * . . . H 
                  5 | * . . . . . . . . . 
                  4 | . . . * . . . * . . 
                  3 | . K . . . . . . . . 
                  2 | . . . * . . . . . * 
                  1 | . . * . . . . * . . 
                  0 ----------------------
                                        1 
                    0 1 2 3 4 5 6 7 8 9 0 
    

The Knight 可以按照下图中的 A,B,C,D…A,B,C,D… 这条路径用 55 次跳到草的地方（有可能其它路线的长度也是 55）：

                 11 | . . . . . . . . . .
                 10 | . . . . * . . . . .
                  9 | . . . . . . . . . .
                  8 | . . . * . * . . . .
                  7 | . . . . . . . * . .
                  6 | . . * . . * . . . F<
                  5 | * . B . . . . . . .
                  4 | . . . * C . . * E .
                  3 | .>A . . . . D . . .
                  2 | . . . * . . . . . *
                  1 | . . * . . . . * . .
                  0 ----------------------
                                        1
                    0 1 2 3 4 5 6 7 8 9 0
    

**注意：** 数据保证一定有解。

#### 输入格式

第 11 行： 两个数，表示农场的列数 CC 和行数 RR。

第 2..R+12..R+1 行: 每行一个由 CC 个字符组成的字符串，共同描绘出牧场地图。

#### 输出格式

一个整数，表示跳跃的最小次数。

#### 数据范围

1≤R,C≤1501≤R,C≤150

#### 输入样例：

    10 11
    ..........
    ....*.....
    ..........
    ...*.*....
    .......*..
    ..*..*...H
    *.........
    ...*...*..
    .K........
    ...*.....*
    ..*....*..
    

#### 输出样例：

    5
    

难度：简单

时/空限制：1s / 64MB

总通过数：15011

总尝试数：28956

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3739&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3739&show_algorithm_tags=1)[广度优先搜索](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%BF%E5%BA%A6%E4%BC%98%E5%85%88%E6%90%9C%E7%B4%A2&source_file_id=3739&show_algorithm_tags=1)[BFS](https://www.acwing.com/problem/search/1/?search_content=BFS&source_file_id=3739&show_algorithm_tags=1)