390\. 逃学的小孩

*    [题目](https://www.acwing.com/problem/content/description/392/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/392/1/)
*    [题解](https://www.acwing.com/problem/content/solution/392/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/392/)

  

克里斯再次逃学去朋友家里玩了，生气的克里斯的父母决定把他给捉回来。

他的父母深知克里斯一定是在夏尔米或者七枷社家里玩。

克里斯所在的城市由 NN 个居住点和 MM 条连接居住点的双向街道组成，经过街道 xx 需要花费 TxTx 分钟。

可以保证，任意两个居住点之间有且仅有一条通路。

克里斯家在点 CC，夏尔米和七枷社家分别在点 AA 和点 BB。

为了尽快找到克里斯，他的父母在寻找他时将遵守如下两条规则：

1.  如果 AA 距离 CC 比 BB 距离 CC 近，则他的父母先到夏尔米家去找他，如果找不到，再去七枷社家。反之亦然。
2.  克里斯的父母总沿着两点间唯一的通路行走。

但是我们并不知道 A、B、CA、B、C 三个点的具体位置。

请你计算在最坏的情况下，克里斯的父母要花多久才能找到他？

#### 输入格式

第一行包含两个整数 NN 和 MM。

接下来 MM 行，每行包含三个整数 u，v，tu，v，t，表示居住点 uu 和居住点 vv 之间存在一条街道，且经过该街道需要花费 tt 分钟。

街道信息不会重复给出。

#### 输出格式

输出一个整数，表示克里斯父母在最坏的情况下，找到他需要花费的分钟数。

#### 数据范围

3≤N≤2000003≤N≤200000,  
M\=N−1M\=N−1,  
1≤u,v≤N1≤u,v≤N,  
1≤t≤10000000001≤t≤1000000000

#### 输入样例：

    4 3
    1 2 1
    2 3 1
    3 4 1
    

#### 输出样例：

    4
    

难度：中等

时/空限制：1s / 64MB

总通过数：543

总尝试数：1440

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3941&show_algorithm_tags=0)[NOI2003](https://www.acwing.com/problem/search/1/?search_content=NOI2003&source_file_id=3941&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3941&show_algorithm_tags=1)[树的直径](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E7%9A%84%E7%9B%B4%E5%BE%84&source_file_id=3941&show_algorithm_tags=1)