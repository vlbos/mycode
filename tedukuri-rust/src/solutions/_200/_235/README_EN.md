235\. 魔法珠

+     [题目](https://www.acwing.com/problem/content/description/237/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/237/1/)
+     [题解](https://www.acwing.com/problem/content/solution/237/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/237/)

  

Freda 和 rainbow 是超自然之界学校(Preternatural Kingdom University，简称PKU)魔法学院的学生。

为了展示新学的魔法，他们决定进行一场对弈。

起初 Freda 面前有 $n$ 堆魔法珠，其中第 $i$ 堆有 $a\_i$ 颗。

Freda 和 rainbow 可以轮流进行以下操作：

1.  选择 $n$ 堆中魔法珠数量大于 $1$ 的任意一堆。记该堆魔法珠的数量为 $p$，$p$ 有 $b\_1、b\_2……b\_m$ 这 $m$ 个小于 $p$ 的约数。
2.  施展魔法把这一堆魔法珠变成 $m$ 堆，每堆各有 $b\_1、b\_2……b\_m$ 颗魔法珠。
3.  选择这 $m$ 堆中的一堆魔法珠，施展魔法令其消失。

注意一次操作过后，魔法珠的堆数会增加 $m-2$，各堆中魔法珠数量的总和可能会发生变化。

当轮到某人操作时，如果每堆中魔法珠的数量均为 $1$，那么他就输了。

Freda 和 rainbow 都采取最好的策略，从 Freda 开始。

请你预测一下，谁能获胜呢？

#### 输入格式

输入包含多组测试数据。

每组数据的第一行包含一个整数 $n$。

第二行包含 $n$ 个整数，第 $i$ 个整数表示第 $i$ 堆的魔法珠数量 $a\_i$。

#### 输出格式

对于每组数据，在两人均采取最佳策略的前提下，若 Freda 能获胜，输出 `freda`；若 Rainbow 能获胜，输出 `rainbow`。

每个结果占一行。

#### 数据范围

$1 \\le n \\le 100$,  
$1 \\le a\_i \\le 1000$

#### 输入样例：

```
3
2 2 2
3
1 3 5
```

#### 输出样例：

```
freda
rainbow
```

* * *