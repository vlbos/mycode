269\. Fotile模拟赛L

+     [题目](https://www.acwing.com/problem/content/description/271/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/271/1/)
+     [题解](https://www.acwing.com/problem/content/solution/271/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/271/)

  

FOTILE 得到了一个长为 $N$ 的序列 $A$，为了拯救地球，他希望知道某些区间内的最大的连续 $XOR$ 和。

即对于一个询问，你需要求出 $max(A\_i\\ xor\\ A\_{i+1}\\ xor\\ A\_{i+2}\\ …\\ xor\\ A\_j)$，其中 $l \\le i \\le j \\le r$。

为了体现在线操作，对于一个询问 $(x,y)$：

+   $l = min ( ((x+lastans) \\bmod N)+1 , ((y+lastans) \\bmod N)+1 )$
+   $r = max ( ((x+lastans) \\bmod N)+1 , ((y+lastans) \\bmod N)+1 )$

其中 $lastans$ 是上次询问的答案，一开始为 $0$。

#### 输入格式

第一行两个整数 $N$ 和 $M$。

第二行有 $N$ 个正整数，其中第 $i$ 个数为 $A\_i$。

后 $M$ 行每行两个整数 $x,y$ 表示一对询问。

#### 输出格式

共 $M$ 行，每行输出一个正整数，第 $i$ 行的正整数表示第 $i$ 个询问的结果。

#### 数据范围

$N=12000，M=6000，0 < A\_i < 2^{31}，0 \\le x,y < 2^{31}$

#### 输入样例：

```
3 3
1 4 3
0 1
0 1
4 3
```

#### 输出样例：

```
5
7
7
```

* * *