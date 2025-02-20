281\. 硬币

+     [题目](https://www.acwing.com/problem/content/description/283/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/283/1/)
+     [题解](https://www.acwing.com/problem/content/solution/283/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/283/)

  

给定 $N$ 种硬币，其中第 $i$ 种硬币的面值为 $A\_i$，共有 $C\_i$ 个。

从中选出若干个硬币，把面值相加，若结果为 $S$，则称“面值 $S$ 能被拼成”。

求 $1 \\sim M$ 之间能被拼成的面值有多少个。

#### 输入格式

输入包含多组测试用例。

每组测试用例第一行包含两个整数 $N$ 和 $M$。

第二行包含 $2N$ 个整数，分别表示 $A\_1,A\_2,…,A\_N$ 和 $C\_1,C\_2,…,C\_N$。

当输入用例 $N=0，M=0$ 时，表示输入终止，且该用例无需处理。

#### 输出格式

每组用例输出一个结果，每个结果占一行。

#### 数据范围

$1 \\le N \\le 100$,  
$1 \\le M \\le 10^5$,  
$1 \\le A\_i \\le 10^5$,  
$1 \\le C\_i \\le 1000$

#### 输入用例：

```
3 10
1 2 4 2 1 1
2 5
1 4 2 1
0 0
```

#### 输出用例：

```
8
4
```

* * *