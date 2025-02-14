289\. 环路运输

+     [题目](https://www.acwing.com/problem/content/description/291/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/291/1/)
+     [题解](https://www.acwing.com/problem/content/solution/291/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/291/)

  

在一条环形公路旁均匀地分布着 $N$ 座仓库，编号为 $1 \\sim N$，编号为 $i$ 的仓库与编号为 $j$ 的仓库之间的距离定义为 $dist(i,j)=\\min⁡(|i-j|,N-|i-j|)$，也就是逆时针或顺时针从 $i$ 到 $j$ 中较近的一种。

每座仓库都存有货物，其中编号为 $i$ 的仓库库存量为 $A\_i$。

在 $i$ 和 $j$ 两座仓库之间运送货物需要的代价为 $A\_i+A\_j+dist(i,j)$。

求在哪两座仓库之间运送货物需要的代价最大。

#### 输入格式

第一行包含一个整数 $N$。

第二行包含 $N$ 个整数 $A\_1 \\sim A\_N$。

#### 输出格式

输出一个整数，表示最大代价。

#### 数据范围

$2 \\le N \\le 10^6$,  
$1 \\le A\_i \\le 10^7$

#### 输入样例：

```
5
1 8 6 2 5
```

#### 输出样例：

```
15
```

* * *