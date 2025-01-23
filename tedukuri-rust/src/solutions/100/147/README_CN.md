147\. 数据备份

*    [题目](https://www.acwing.com/problem/content/description/149/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/149/1/)
*    [题解](https://www.acwing.com/problem/content/solution/149/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/149/)

  

你在一家 IT 公司为大型写字楼或办公楼的计算机数据做备份。

然而数据备份的工作是枯燥乏味的，因此你想设计一个系统让不同的办公楼彼此之间互相备份，而你则坐在家中尽享计算机游戏的乐趣。

已知办公楼都位于同一条街上，你决定给这些办公楼配对（两个一组）。

每一对办公楼可以通过在这两个建筑物之间铺设网络电缆使得它们可以互相备份。

然而，网络电缆的费用很高。

当地电信公司仅能为你提供 KK 条网络电缆，这意味着你仅能为 KK 对办公楼（总计 2K2K 个办公楼）安排备份。

任意一个办公楼都属于唯一的配对组（换句话说，这 2K2K 个办公楼一定是相异的）。

此外，电信公司需按网络电缆的长度（公里数）收费。

因而，你需要选择这 KK 对办公楼使得电缆的总长度尽可能短。

换句话说，你需要选择这 KK 对办公楼，使得每一对办公楼之间的距离之和（总距离）尽可能小。

下面给出一个示例，假定你有 55 个客户，其办公楼都在一条街上，如下图所示。

这 55 个办公楼分别位于距离大街起点 1km,3km,4km,6km1km,3km,4km,6km 和 12km12km 处。

电信公司仅为你提供 K\=2K\=2 条电缆。

![1111.png](https://cdn.acwing.com/media/article/image/2019/01/15/19_131a625c18-1111.png)

上例中最好的配对方案是将第 11 个和第 22 个办公楼相连，第 33 个和第 44 个办公楼相连。

这样可按要求使用 K\=2K\=2 条电缆。

第 11 条电缆的长度是 3km−1km\=2km3km−1km\=2km，第 22 条电缆的长度是 6km−4km\=2km6km−4km\=2km。

这种配对方案需要总长 4km4km 的网络电缆，满足距离之和最小的要求。

#### 输入格式

第一行输入整数 nn 和 KK，其中 nn 表示办公楼的数目，KK 表示可利用的网络电缆的数目。

接下来的 nn 行每行仅包含一个整数 ss，表示每个办公楼到大街起点处的距离。

这些整数将按照从小到大的顺序依次出现。

#### 输出格式

输出应由一个正整数组成，给出将 2K2K 个相异的办公楼连成 KK 对所需的网络电缆的最小总长度。

#### 数据范围

2≤n≤1000002≤n≤100000,  
1≤K≤n/21≤K≤n/2,  
0≤s≤10000000000≤s≤1000000000

#### 输入样例：

    5 2 
    1
    3
    4
    6
    12
    

#### 输出样例：

    4
    

难度：困难

时/空限制：1s / 64MB

总通过数：2579

总尝试数：7555

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3698&show_algorithm_tags=0)

算法标签

[二叉堆](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%8F%89%E5%A0%86&source_file_id=3698&show_algorithm_tags=1)[双链表](https://www.acwing.com/problem/search/1/?search_content=%E5%8F%8C%E9%93%BE%E8%A1%A8&source_file_id=3698&show_algorithm_tags=1)[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3698&show_algorithm_tags=1)