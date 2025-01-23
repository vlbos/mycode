195\. 骑士精神

*    [题目](https://www.acwing.com/problem/content/description/197/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/197/1/)
*    [题解](https://www.acwing.com/problem/content/solution/197/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/197/)

  

在一个 5×55×5 的棋盘上有 1212 个白色的骑士和 1212 个黑色的骑士，且有一个空位。

在任何时候一个骑士都能按照骑士的走法（它可以走到和它横坐标相差为 11，纵坐标相差为 22 或者横坐标相差为 22，纵坐标相差为 11 的格子）移动到空位上。

给定一个初始的棋盘，怎样才能经过移动变成如下目标棋盘：为了体现出骑士精神，他们必须以最少的步数完成任务。

![aa.jpg.gif](https://cdn.acwing.com/media/article/image/2019/01/17/19_710140aa19-aa.jpg.gif)

#### 输入格式

第一行有一个正整数 TT，表示一共有 TT 组数据。

接下来有 TT 个 5×55×5 的矩阵，00 表示白色骑士，11 表示黑色骑士，`*` 表示空位。

两组数据之间没有空行。

#### 输出格式

每组数据输出占一行。

如果能在 1515 步以内（包括 1515 步）到达目标状态，则输出步数，否则输出 −1−1。

#### 数据范围

1≤T≤101≤T≤10

#### 输入样例：

    2
    10110
    01*11
    10111
    01001
    00000
    01011
    110*1
    01110
    01010
    00100
    

#### 输出样例：

    7
    -1
    

难度：简单

时/空限制：1s / 64MB

总通过数：1173

总尝试数：2217

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3746&show_algorithm_tags=0)[SCOI2005](https://www.acwing.com/problem/search/1/?search_content=SCOI2005&source_file_id=3746&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3746&show_algorithm_tags=1)[IDA\*](https://www.acwing.com/problem/search/1/?search_content=IDA*&source_file_id=3746&show_algorithm_tags=1)