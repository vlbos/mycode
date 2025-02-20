5579\. 增加模数

+     [题目](https://www.acwing.com/problem/content/description/5582/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/5582/1/)
+     [题解](https://www.acwing.com/problem/content/solution/5582/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/5582/)

  

给定 $H$ 对非负整数数对 $(A\_i,B\_i)$ 和一个正整数 $M$。

请你计算并输出 $(A\_1^{B\_1}+A\_2^{B\_2}+…+A\_H^{B\_H}) \\bmod M$。

#### 输入格式

第一行包含整数 $T$，表示共有 $T$ 组测试数据。

每组数据第一行包含整数 $M$。

第二行包含整数 $H$。

接下来 $H$ 行，每行包含两个整数 $A\_i,B\_i$。

#### 输出格式

每组数据输出一行结果。

#### 数据范围

$1 \\le T \\le 100$,  
$1 \\le M \\le 45000$,  
$1 \\le H \\le 45000$,  
$0 \\le A\_i,B\_i \\le 10^7$,  
$A\_i$ 和 $B\_i$ 不同时为 $0$。

#### 输入样例：

```
3
16
4
2 3
3 4
4 5
5 6
36123
1
2374859 3029382
17
1
3 18132
```

#### 输出样例：

```
2
13195
13
```

* * *