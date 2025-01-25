290\. 坏掉的机器人

*    [题目](https://www.acwing.com/problem/content/description/292/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/292/1/)
*    [题解](https://www.acwing.com/problem/content/solution/292/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/292/)

  

给定一张 N×MN×M 的棋盘，有一个机器人处于 (x,y)(x,y) 位置。

这个机器人可以进行很多轮行动，每次等概率地随机选择停在原地、向左移动一格、向右移动一格或向下移动一格。

当然机器人不能移出棋盘。

求机器人从起点走到最后一行的任意一个位置上，所需行动次数的数学期望值。

#### 输入格式

第一行包含两个整数 NN 和 MM。

第二行包含两个整数 xx 和 yy，表示机器人的初始位置。

设定棋盘左上角为 (1,1)(1,1)，右下角为 (N,M)(N,M)。

#### 输出格式

输出一个实数，表示数学期望，结果保留四位小数。

#### 数据范围

1≤N,M≤10001≤N,M≤1000

#### 输入样例：

    10 14 
    5 14
    

#### 输出样例：

    18.0038
    

难度：困难

时/空限制：1s / 64MB

总通过数：935

总尝试数：1981

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3841&show_algorithm_tags=0)[CF24D](https://www.acwing.com/problem/search/1/?search_content=CF24D&source_file_id=3841&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3841&show_algorithm_tags=1)[后效性处理](https://www.acwing.com/problem/search/1/?search_content=%E5%90%8E%E6%95%88%E6%80%A7%E5%A4%84%E7%90%86&source_file_id=3841&show_algorithm_tags=1)