205\. 斐波那契

+     [题目](https://www.acwing.com/problem/content/description/207/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/207/1/)
+     [题解](https://www.acwing.com/problem/content/solution/207/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/207/)

  

在斐波那契数列中，$Fib\_0=0, Fib\_1=1, Fib\_n=Fib\_{n-1}+Fib\_{n-2} (n>1)$。

给定整数 $n$，求 $Fib\_n \\bmod 10000$。

#### 输入格式

输入包含不超过 $100$ 组测试用例。

每个测试用例占一行，包含一个整数 $n$。

当输入用例 $n=-1$ 时，表示输入终止，且该用例无需处理。

#### 输出格式

每个测试用例输出一个整数表示结果。

每个结果占一行。

#### 数据范围

$0 \\le n \\le 2 \\times 10^9$

#### 输入样例：

```
0
9
999999999
1000000000
-1
```

#### 输出样例：

```
0
34
626
6875
```

* * *