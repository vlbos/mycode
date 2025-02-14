377\. 泥泞的区域

+     [题目](https://www.acwing.com/problem/content/description/379/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/379/1/)
+     [题解](https://www.acwing.com/problem/content/solution/379/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/379/)

  

在一块 $N \\times M$ 的网格状地面上，有一些格子是泥泞的，其他格子是干净的。

现在需要用一些宽度为 $1$、长度任意的木板把泥地盖住，同时不能盖住干净的地面。

每块木板必须覆盖若干个完整的格子，木板可以重叠。

求最少需要多少木板。

#### 输入格式

第一行包含两个整数 $N$ 和 $M$。

接下来 $N$ 行，每行 $M$ 个字符，用来描述地面，`*` 表示泥泞格子，`.` 表示干净格子，字符之间没有空格。

#### 输出格式

输出一个整数，表示结果。

#### 数据范围

$1 \\le N,M \\le 50$

#### 输入样例：

```
4 4
*.*.
.***
***.
..*.
```

#### 输出样例：

```
4
```

* * *