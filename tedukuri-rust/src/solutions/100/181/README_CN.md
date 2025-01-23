181\. 回转游戏

*    [题目](https://www.acwing.com/problem/content/description/183/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/183/1/)
*    [题解](https://www.acwing.com/problem/content/solution/183/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/183/)

  

如下图所示，有一个 `#` 形的棋盘，上面有 1,2,31,2,3 三种数字各 88 个。

给定 88 种操作，分别为图中的 A∼HA∼H。

这些操作会按照图中字母和箭头所指明的方向，把一条长为 77 的序列循环移动 11 个单位。

例如下图最左边的 `#` 形棋盘执行操作 AA 后，会变为下图中间的 `#` 形棋盘，再执行操作 CC 后会变成下图最右边的 `#` 形棋盘。

给定一个初始状态，请使用最少的操作次数，使 `#` 形棋盘最中间的 88 个格子里的数字相同。

![2286_1.jpg](https://cdn.acwing.com/media/article/image/2019/01/23/19_4ec33e321e-2286_1.jpg)

#### 输入格式

输入包含多组测试用例。

每个测试用例占一行，包含 2424 个数字，表示将初始棋盘中的每一个位置的数字，按整体从上到下，同行从左到右的顺序依次列出。

输入样例中的第一个测试用例，对应上图最左边棋盘的初始状态。

当输入只包含一个 00 的行时，表示输入终止。

#### 输出格式

每个测试用例输出占两行。

第一行包含所有移动步骤，每步移动用大写字母 A∼HA∼H 中的一个表示，字母之间没有空格，如果不需要移动则输出 `No moves needed`。

第二行包含一个整数，表示移动完成后，中间 88 个格子里的数字。

如果有多种方案，则输出字典序最小的解决方案。

#### 输入样例：

    1 1 1 1 3 2 3 2 3 1 3 2 2 3 1 2 2 2 3 1 2 1 3 3
    1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 3 3 3 3 3 3 3 3
    0
    

#### 输出样例：

    AC
    2
    DDHH
    2
    

难度：中等

时/空限制：4s / 64MB

总通过数：5275

总尝试数：10216

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3732&show_algorithm_tags=0)[UVA1343](https://www.acwing.com/problem/search/1/?search_content=UVA1343&source_file_id=3732&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3732&show_algorithm_tags=1)[IDA\*](https://www.acwing.com/problem/search/1/?search_content=IDA*&source_file_id=3732&show_algorithm_tags=1)