299\. 裁剪序列

+     [题目](https://www.acwing.com/problem/content/description/301/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/301/1/)
+     [题解](https://www.acwing.com/problem/content/solution/301/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/301/)

  

给定一个长度为 $N$ 的序列 $A$，要求把该序列分成若干段，在满足“每段中所有数的和”不超过 $M$ 的前提下，让“每段中所有数的最大值”之和最小。

试计算这个最小值。

#### 输入格式

第一行包含两个整数 $N$ 和 $M$。

第二行包含 $N$ 个整数，表示完整的序列 $A$。

#### 输出格式

输出一个整数，表示结果。

如果结果不存在，则输出 $-1$。

#### 数据范围

$0 \\le N \\le 10^5$,  
$0 \\le M \\le 10^{11}$,  
序列A中的数非负，且不超过$10^6$

#### 输入样例：

```
8 17
2 2 2 8 1 8 2 1
```

#### 输出样例：

```
12
```

* * *