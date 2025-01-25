284\. 金字塔

*    [题目](https://www.acwing.com/problem/content/description/286/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/286/1/)
*    [题解](https://www.acwing.com/problem/content/solution/286/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/286/)

  

虽然探索金字塔是极其老套的剧情，但是有一队探险家还是到了某金字塔脚下。

经过多年的研究，科学家对这座金字塔的内部结构已经有所了解。

首先，金字塔由若干房间组成，房间之间连有通道。

如果把房间看作节点，通道看作边的话，整个金字塔呈现一个有根树结构，节点的子树之间有序，金字塔有唯一的一个入口通向树根。

并且，每个房间的墙壁都涂有若干种颜色的一种。

探险队员打算进一步了解金字塔的结构，为此，他们使用了一种特殊设计的机器人。

这种机器人会从入口进入金字塔，之后对金字塔进行深度优先遍历。

机器人每进入一个房间（无论是第一次进入还是返回），都会记录这个房间的颜色。

最后，机器人会从入口退出金字塔。

显然，机器人会访问每个房间至少一次，并且穿越每条通道恰好两次（两个方向各一次）， 然后，机器人会得到一个颜色序列。

但是，探险队员发现这个颜色序列并不能唯一确定金字塔的结构。

现在他们想请你帮助他们计算，对于一个给定的颜色序列，有多少种可能的结构会得到这个序列。

因为结果可能会非常大，你只需要输出答案对109109 取模之后的值。

#### 输入格式

输入仅一行，包含一个字符串 SS，长度不超过 300300，表示机器人得到的颜色序列。

#### 输出格式

输出一个整数表示答案。

#### 输入样例：

    ABABABA
    

#### 输出样例：

    5
    

难度：中等

时/空限制：1s / 64MB

总通过数：3038

总尝试数：5621

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3835&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3835&show_algorithm_tags=1)[区间DP](https://www.acwing.com/problem/search/1/?search_content=%E5%8C%BA%E9%97%B4DP&source_file_id=3835&show_algorithm_tags=1)