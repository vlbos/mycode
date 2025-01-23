177\. 噩梦

*    [题目](https://www.acwing.com/problem/content/description/179/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/179/1/)
*    [题解](https://www.acwing.com/problem/content/solution/179/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/179/)

  

给定一张 N×MN×M 的地图，地图中有 11 个男孩，11 个女孩和 22 个鬼。

字符 `.` 表示道路，字符 `X` 表示墙，字符 `M` 表示男孩的位置，字符 `G` 表示女孩的位置，字符 `Z` 表示鬼的位置。

男孩每秒可以移动 33 个单位距离，女孩每秒可以移动 11 个单位距离，男孩和女孩只能朝上下左右四个方向移动。

每个鬼占据的区域每秒可以向四周扩张 22 个单位距离，并且无视墙的阻挡，也就是在第 kk 秒后所有与鬼的曼哈顿距离不超过 2k2k 的位置都会被鬼占领。

**注意：** 每一秒鬼会先扩展，扩展完毕后男孩和女孩才可以移动。

求在不进入鬼的占领区的前提下，男孩和女孩能否会合，若能会合，求出最短会合时间。

#### 输入格式

第一行包含整数 TT，表示共有 TT 组测试用例。

每组测试用例第一行包含两个整数 NN 和 MM，表示地图的尺寸。

接下来 NN 行每行 MM 个字符，用来描绘整张地图的状况。（注意：地图中一定有且仅有 11 个男孩，11 个女孩和 22 个鬼）

#### 输出格式

每个测试用例输出一个整数 SS，表示最短会合时间。

如果无法会合则输出 −1−1。

每个结果占一行。

#### 数据范围

1<n,m<8001<n,m<800

#### 输入样例：

    3
    5 6
    XXXXXX
    XZ..ZX
    XXXXXX
    M.G...
    ......
    5 6
    XXXXXX
    XZZ..X
    XXXXXX
    M.....
    ..G...
    10 10
    ..........
    ..X.......
    ..M.X...X.
    X.........
    .X..X.X.X.
    .........X
    ..XX....X.
    X....G...X
    ...ZX.X...
    ...Z..X..X
    

#### 输出样例：

    1
    1
    -1
    

难度：中等

时/空限制：1s / 64MB

总通过数：2694

总尝试数：7458

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3728&show_algorithm_tags=0)[HDU3085](https://www.acwing.com/problem/search/1/?search_content=HDU3085&source_file_id=3728&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3728&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3728&show_algorithm_tags=1)[双向BFS](https://www.acwing.com/problem/search/1/?search_content=%E5%8F%8C%E5%90%91BFS&source_file_id=3728&show_algorithm_tags=1)