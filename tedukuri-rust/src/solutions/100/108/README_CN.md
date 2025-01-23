108\. 奇数码问题

*    [题目](https://www.acwing.com/problem/content/description/110/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/110/1/)
*    [题解](https://www.acwing.com/problem/content/solution/110/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/110/)

  

你一定玩过八数码游戏，它实际上是在一个 3×33×3 的网格中进行的，11 个空格和 1∼81∼8 这 88 个数字恰好不重不漏地分布在这 3×33×3 的网格中。

例如：

    5 2 8
    1 3 _
    4 6 7
    

在游戏过程中，可以把空格与其上、下、左、右四个方向之一的数字交换（如果存在）。

例如在上例中，空格可与左、上、下面的数字交换，分别变成：

    5 2 8       5 2 _      5 2 8
    1 _ 3       1 3 8      1 3 7
    4 6 7       4 6 7      4 6 _
    

奇数码游戏是它的一个扩展，在一个 n×nn×n 的网格中进行，其中 nn 为奇数，11 个空格和 1∼n2−11∼n2−1 这 n2−1n2−1 个数恰好不重不漏地分布在 n×nn×n 的网格中。

空格移动的规则与八数码游戏相同，实际上，八数码就是一个 n\=3n\=3 的奇数码游戏。

现在给定两个奇数码游戏的局面，请判断是否存在一种移动空格的方式，使得其中一个局面可以变化到另一个局面。

#### 输入格式

多组数据，对于每组数据：

第 11 行输入一个整数 nn，nn 为奇数。

接下来 nn 行每行 nn 个整数，表示第一个局面。

再接下来 nn 行每行 nn 个整数，表示第二个局面。

局面中每个整数都是 0∼n2−10∼n2−1 之一，其中用 00 代表空格，其余数值与奇数码游戏中的意义相同，保证这些整数的分布不重不漏。

#### 输出格式

对于每组数据，若两个局面可达，输出 `TAK`，否则输出 `NIE`。

#### 数据范围

1≤n<5001≤n<500

#### 输入样例：

    3
    1 2 3
    0 4 6
    7 5 8
    1 2 3
    4 5 6
    7 8 0
    1
    0
    0
    

#### 输出样例：

    TAK
    TAK
    

难度：中等

时/空限制：2s / 64MB

总通过数：4633

总尝试数：17149

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3659&show_algorithm_tags=0)

算法标签

[逆序对](https://www.acwing.com/problem/search/1/?search_content=%E9%80%86%E5%BA%8F%E5%AF%B9&source_file_id=3659&show_algorithm_tags=1)