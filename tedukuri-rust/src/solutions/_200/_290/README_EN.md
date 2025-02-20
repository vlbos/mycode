290\. 坏掉的机器人

+     [题目](https://www.acwing.com/problem/content/description/292/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/292/1/)
+     [题解](https://www.acwing.com/problem/content/solution/292/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/292/)

  

给定一张 $N \\times M$ 的棋盘，有一个机器人处于 $(x,y)$ 位置。

这个机器人可以进行很多轮行动，每次等概率地随机选择停在原地、向左移动一格、向右移动一格或向下移动一格。

当然机器人不能移出棋盘。

求机器人从起点走到最后一行的任意一个位置上，所需行动次数的数学期望值。

#### 输入格式

第一行包含两个整数 $N$ 和 $M$。

第二行包含两个整数 $x$ 和 $y$，表示机器人的初始位置。

设定棋盘左上角为 $(1,1)$，右下角为 $(N,M)$。

#### 输出格式

输出一个实数，表示数学期望，结果保留四位小数。

#### 数据范围

$1 \\le N,M \\le 1000$

#### 输入样例：

```
10 14 
5 14
```

#### 输出样例：

```
18.0038
```

* * *