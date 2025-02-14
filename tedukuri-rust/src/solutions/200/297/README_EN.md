297\. 赤壁之战

+     [题目](https://www.acwing.com/problem/content/description/299/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/299/1/)
+     [题解](https://www.acwing.com/problem/content/solution/299/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/299/)

  

给定一个长度为 $N$ 的序列 $A$，求 $A$ 有多少个长度为 $M$ 的严格递增子序列。

#### 输入格式

第一行包含整数 $T$，表示共有 $T$ 组测试数据。

每组数据，第一行包含两个整数 $N$ 和 $M$。

第二行包含 $N$ 个整数，表示完整的序列 $A$。

#### 输出格式

每组数据输出一个结果，每个结果占一行。

输出格式为 `Case #x: y`，$x$ 为数据组别序号，从 $1$ 开始，$y$ 为结果。

由于答案可能很大，请你输出对 $10^9+7$ 取模后的结果。

#### 数据范围

$1 \\le T \\le 100$,  
$1 \\le M \\le N \\le 1000$,  
$\\sum\_{i=1}^T N\_i \\times M\_i \\le 10^7$  
序列中的整数的绝对值不超过$10^9$。

#### 输入样例：

```
2
3 2
1 2 3
3 2
3 2 1
```

#### 输出样例：

```
Case #1: 3
Case #2: 0
```

* * *