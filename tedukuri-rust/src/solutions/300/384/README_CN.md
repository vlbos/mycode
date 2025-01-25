384\. 升降梯上

*    [题目](https://www.acwing.com/problem/content/description/386/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/386/1/)
*    [题解](https://www.acwing.com/problem/content/solution/386/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/386/)

  

开启了升降梯的动力之后，探险队员们进入了升降梯运行的那条竖直的隧道，映入眼帘的是一条直通塔顶的轨道、一辆停在轨道底部的电梯、和电梯内一杆控制电梯升降的巨大手柄。

NescaféNescafé 之塔一共有 NN 层，升降梯在每层都有一个停靠点。

手柄有 MM 个控制槽，第 ii 个控制槽旁边标着一个数 CiCi，满足 C1<C2<C3<…<CMC1<C2<C3<…<CM。

如果 Ci\>0Ci\>0，表示手柄扳动到该槽时，电梯将上升 CiCi 层；如果 Ci<0Ci<0，表示手柄扳动到该槽时，电梯将下降 −Ci−Ci 层；并且一定存在一个 Ci\=0Ci\=0，手柄最初就位于此槽中。

注意升降梯只能在 1∼N1∼N 层间移动，因此扳动到使升降梯移动到 11 层以下、NN 层以上的控制槽是不允许的。

电梯每移动一层，需要花费 22 秒钟时间，而手柄从一个控制槽扳到相邻的槽，需要花费 11 秒钟时间。

探险队员现在在 11 层，并且想尽快到达 NN 层，他们想知道从 11 层到 NN 层至少需要多长时间？

#### 输入格式

第一行两个正整数 N、MN、M。

第二行 MM 个整数 C1、C2…CMC1、C2…CM。

#### 输出格式

输出一个整数表示答案，即至少需要多长时间。

若不可能到达输出 −1−1。

#### 数据范围

1≤N≤10001≤N≤1000,  
2≤M≤202≤M≤20,  
−N<C1<C2<…<CM<N−N<C1<C2<…<CM<N

#### 输入样例：

    6 3
    -1 0 2
    

#### 输出样例：

    19
    

难度：简单

时/空限制：1s / 64MB

总通过数：633

总尝试数：1391

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3935&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3935&show_algorithm_tags=1)[最短路](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E7%9F%AD%E8%B7%AF&source_file_id=3935&show_algorithm_tags=1)[节点扩展到二维](https://www.acwing.com/problem/search/1/?search_content=%E8%8A%82%E7%82%B9%E6%89%A9%E5%B1%95%E5%88%B0%E4%BA%8C%E7%BB%B4&source_file_id=3935&show_algorithm_tags=1)