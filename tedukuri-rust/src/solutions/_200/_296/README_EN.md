296\. 清理班次2

+     [题目](https://www.acwing.com/problem/content/description/298/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/298/1/)
+     [题解](https://www.acwing.com/problem/content/solution/298/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/298/)

  

农夫约翰雇佣他的 $N$ 头奶牛帮他进行牛棚的清理工作。

他将全天分为了很多个班次，其中第 $M$ 个班次到第 $E$ 个班次（包括这两个班次）之间必须都有牛进行清理。

这 $N$ 头牛中，第 $i$ 头牛可以从第 $a\_i$ 个班次工作到第 $b\_i$ 个班次，同时，它会索取 $c\_i$ 的佣金。

请你安排一个合理的清理班次，使得 $\[M,E\]$ 时间段内都有奶牛在清理，并且所需支付给奶牛的报酬最少。

#### 输入格式

第 $1$ 行：包含三个整数 $N，M$ 和 $E$。

第 $2..N+1$ 行：第 $i+1$ 行包含三个整数 $a\_i,b\_i,c\_i$。

#### 输出格式

输出一个整数，表示所需的最少佣金。

如果无法做到在要求时间段内都有奶牛清理，则输出 $-1$。

#### 数据范围

$1 \\le N \\le 10000$,  
$0 \\le M,E \\le 86399$,  
$M \\le a\_i \\le b\_i \\le E$,  
$0 \\le c\_i \\le 500000$

#### 输入样例：

```
3 0 4
0 2 3
3 4 2
0 0 1
```

#### 输出样例：

```
5
```

* * *