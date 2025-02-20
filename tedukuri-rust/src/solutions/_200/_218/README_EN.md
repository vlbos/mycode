218\. 扑克牌

+     [题目](https://www.acwing.com/problem/content/description/220/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/220/1/)
+     [题解](https://www.acwing.com/problem/content/solution/220/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/220/)

  

Admin 生日那天，Rainbow 来找 Admin 玩扑克牌。

玩着玩着 Rainbow 觉得太没意思了，于是决定给 Admin 一个考验。

Rainbow 把一副扑克牌($54$ 张)随机洗开，倒扣着放成一摞。

然后 Admin 从上往下依次翻开每张牌，每翻开一张黑桃、红桃、梅花或者方块，就把它放到对应花色的堆里去。

Rainbow 想问问 Admin，得到至少 $A$ 张黑桃、至少 $B$ 张红桃、至少 $C$ 张梅花、至少 $D$ 张方块需要翻开的牌的张数的期望值 $E$ 是多少？

特殊地，如果翻开的牌是大王或者小王，Admin 将会把它作为某种花色的牌放入对应堆中，使得放入之后 $E$ 的值尽可能小。

由于 Admin 和 Rainbow 还在玩扑克，所以这个程序就交给你来写了。

#### 输入格式

输入仅由一行，包含四个用空格隔开的整数，$A,B,C,D$。

#### 输出格式

输出需要翻开的牌数的期望值 $E$，四舍五入保留 $3$ 位小数。

如果不可能达到输入的状态，输出 `-1.000`。

#### 数据范围

$0 \\le A,B,C,D \\le 15$

#### 输入样例：

```
1 2 3 4
```

#### 输出样例：

```
16.393
```

* * *