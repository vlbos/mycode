354\. 天天爱跑步

*    [题目](https://www.acwing.com/problem/content/description/356/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/356/1/)
*    [题解](https://www.acwing.com/problem/content/solution/356/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/356/)

  

小 CC 同学认为跑步非常有趣，于是决定制作一款叫作《天天爱跑步》的游戏。

《天天爱跑步》是一个养成类游戏，需要玩家每天按时上线，完成打卡任务。

这个游戏的地图可以看作一棵包含 nn 个节点和 n−1n−1 条边的树，任意两个节点存在一条路径互相可达。

树上节点的编号是 1∼n1∼n 之间的连续正整数。

现在有 mm 个玩家，第 ii 个玩家的起点为 SiSi，终点为 TiTi。

每天打卡任务开始时，所有玩家在第 00 秒同时从自己的起点出发，以每秒跑一条边的速度，不间断地沿着最短路径向着自己的终点跑去，跑到终点后该玩家就算完成了打卡任务。

因为地图是一棵树，所以每个人的路径是唯一的。

小 CC 想知道游戏的活跃度，所以在每个节点上都放置了一个观察员。

在节点 jj 的观察员会选择在第 WjWj 秒观察玩家，一个玩家能被这个观察员观察到当且仅当该玩家在第 WjWj 秒也正好到达了节点 jj。

小 CC 想知道每个观察员会观察到多少人？

注意：我们认为一个玩家到达自己的终点后，该玩家就会结束游戏，他不能等待一段时间后再被观察员观察到。

即对于把节点 jj 作为终点的玩家：若他在第 WjWj 秒前到达终点，则在节点 jj 的观察员不能观察到该玩家；若他正好在第 WjWj 秒到达终点，则在节点 jj 的观察员可以观察到这个玩家。

#### 输入格式

第一行有两个整数 nn 和 mm。

其中 nn 代表树的结点数量，同时也是观察员的数量，mm 代表玩家的数量。

接下来 n−1n−1 行每行两个整数 UU 和 VV，表示结点 UU 到结点 VV 有一条边。

接下来一行 nn 个整数，其中第个整数为 WjWj，表示结点出现观察员的时间。

接下来 mm 行，每行两个整数 SiSi 和 TiTi，表示一个玩家的起点和终点。

#### 输出格式

一行 nn 个整数，第 ii 个整数表示结点 ii 的观察员可以观察到多少人。

#### 数据范围

1≤n,m≤3×1051≤n,m≤3×105  
1≤Si,Ti≤n1≤Si,Ti≤n  
0≤Wj≤n0≤Wj≤n

#### 输入样例：

    6 3
    2 3
    1 2 
    1 4 
    4 5 
    4 6 
    0 2 5 1 2 3 
    1 5 
    1 3 
    2 6
    

#### 输出样例：

    2 0 0 1 1 1
    

难度：中等

时/空限制：2s / 512MB

总通过数：1114

总尝试数：2898

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3905&show_algorithm_tags=0)[NOIP2016提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2016%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3905&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3905&show_algorithm_tags=1)[LCA](https://www.acwing.com/problem/search/1/?search_content=LCA&source_file_id=3905&show_algorithm_tags=1)[树上差分](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E4%B8%8A%E5%B7%AE%E5%88%86&source_file_id=3905&show_algorithm_tags=1)