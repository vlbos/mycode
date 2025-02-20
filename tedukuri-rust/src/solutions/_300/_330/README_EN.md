330\. 估算

+     [题目](https://www.acwing.com/problem/content/description/332/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/332/1/)
+     [题解](https://www.acwing.com/problem/content/solution/332/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/332/)

  

给定一个长度为 $N$ 的整数数组 $A$，你需要创建另一个长度为 $N$ 的整数数组 $B$，数组 $B$ 被分为 $K$ 个连续的部分，并且如果 $i$ 和 $j$ 在同一个部分，则 $B\[i\]=B\[j\]$。

如果要求数组 $B$ 能够满足 $Σ|A\[i\]-B\[i\]|$ 最小，那么最小值是多少，请你输出这个最小值。

#### 输入格式

输入包含不超过 $25$ 组测试数据。

对于每组测试数据，第一行包含两个整数 $N$ 和 $K$。

接下来 $N$ 行每行包含一个整数，表示完整的数组 $A$。

当输入为一行 `0 0` 时，表示输入终止。

#### 输出格式

对于每组数据，输出一个最小值。

每个结果占一行。

#### 数据范围

$1 \\le N \\le 2000$,  
$1 \\le K \\le 25,K \\le N$  
数组 $A$ 中元素的绝对值不超过 $10000$。

#### 输入样例：

```
7 2
6
5
4
3
2
1
7
0 0
```

#### 输出样例：

```
9
```

* * *