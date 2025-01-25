406\. 放置机器人

*    [题目](https://www.acwing.com/problem/content/description/408/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/408/1/)
*    [题解](https://www.acwing.com/problem/content/solution/408/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/408/)

  

给出一个地图（网格），格子分为空地，草地，墙壁。

要在空地上放能向上下左右 44 个方向发射激光的机器人。

墙壁能挡住激光，草地不能挡住激光也不能放机器人。

在机器人不能互相打到对方的情况下，最多放置多少个机器人。

#### 输入格式

第一行包含整数 TT，表示共有 TT 组测试数据。

每组数据第一行包含两个整数 mm 和 nn，表示地图的大小为 mm 行 nn 列。

接下来 mm 行，每行包含 nn 个字符，用来描述整个地图。

`#` 代表墙壁，`*` 代表草地，`o` 代表空地。

#### 输出格式

每组测试数据在第一行输出 `Case :id`，`id` 是数据编号，从 11 开始。

第二行包含一个整数，表示机器人的个数。

#### 数据范围

1≤m,n≤501≤m,n≤50

#### 输入样例：

    2
    4 4
    o***
    *###
    oo#o
    ***o
    4 4
    #ooo
    o#oo
    oo#o
    ***#
    

#### 输出样例：

    Case :1
    3
    Case :2
    5
    

难度：中等

时/空限制：1s / 64MB

总通过数：336

总尝试数：804

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3957&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3957&show_algorithm_tags=1)[二分图最大匹配](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86%E5%9B%BE%E6%9C%80%E5%A4%A7%E5%8C%B9%E9%85%8D&source_file_id=3957&show_algorithm_tags=1)